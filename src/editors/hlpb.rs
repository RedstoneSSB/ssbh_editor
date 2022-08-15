use std::path::Path;

use egui::{CollapsingHeader, DragValue, Grid, ScrollArea, Ui};
use log::error;
use rfd::FileDialog;
use ssbh_data::{prelude::*, Vector3, Vector4};

use crate::widgets::{bone_combo_box, DragSlider};

pub fn hlpb_editor(
    ctx: &egui::Context,
    title: &str,
    folder_name: &str,
    file_name: &str,
    hlpb: &mut HlpbData,
    skel: Option<&SkelData>,
) -> (bool, bool) {
    let mut open = true;
    let mut changed = true;

    egui::Window::new(format!("Hlpb Editor ({title})"))
        .open(&mut open)
        .resizable(true)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Save").clicked() {
                        ui.close_menu();

                        let file = Path::new(folder_name).join(file_name);
                        if let Err(e) = hlpb.write_to_file(&file) {
                            error!("Failed to save {:?}: {}", file, e);
                        }
                    }

                    if ui.button("Save As...").clicked() {
                        ui.close_menu();

                        if let Some(file) = FileDialog::new()
                            .add_filter("Hlpb", &["nuhlpb"])
                            .save_file()
                        {
                            if let Err(e) = hlpb.write_to_file(&file) {
                                error!("Failed to save {:?}: {}", file, e);
                            }
                        }
                    }
                });

                egui::menu::menu_button(ui, "Help", |ui| {
                    if ui.button("Hlpb Editor Wiki").clicked() {
                        ui.close_menu();

                        let link = "https://github.com/ScanMountGoat/ssbh_editor/wiki/Hlpb-Editor";
                        if let Err(e) = open::that(link) {
                            log::error!("Failed to open {link}: {e}");
                        }
                    }
                });
            });
            ui.separator();

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // TODO: Use a layout similar to the matl editor to support more fields.
                    // TODO: Add and delete entries.
                    if !hlpb.aim_constraints.is_empty() {
                        changed |= aim_constraints(ui, hlpb, skel);
                    }

                    if !hlpb.orient_constraints.is_empty() {
                        changed |= orient_constraints(ui, hlpb, skel);
                    }
                });
        });

    (open, changed)
}

fn orient_constraints(ui: &mut Ui, hlpb: &mut HlpbData, skel: Option<&SkelData>) -> bool {
    let mut changed = false;
    CollapsingHeader::new("Orient Constraints")
        .default_open(true)
        .show(ui, |ui| {
            for (i, o) in hlpb.orient_constraints.iter_mut().enumerate() {
                let id = egui::Id::new("orient").with(i);

                // Append the helper bone name to make it easier to find constraints.
                CollapsingHeader::new(format!("{} ({})", o.name, o.target_bone_name))
                    .id_source(id.with(&o.name))
                    .default_open(false)
                    .show(ui, |ui| {
                        Grid::new(id).show(ui, |ui| {
                            ui.label("Name");
                            changed |= ui.text_edit_singleline(&mut o.name).changed();
                            ui.end_row();

                            ui.label("Parent 1");
                            changed |=
                                bone_combo_box(ui, &mut o.parent_bone1_name, id.with(0), skel, &[]);
                            ui.end_row();

                            ui.label("Parent 2");
                            changed |=
                                bone_combo_box(ui, &mut o.parent_bone2_name, id.with(1), skel, &[]);
                            ui.end_row();

                            ui.label("Source");
                            changed |=
                                bone_combo_box(ui, &mut o.source_bone_name, id.with(2), skel, &[]);
                            ui.end_row();

                            ui.label("Target");
                            changed |=
                                bone_combo_box(ui, &mut o.target_bone_name, id.with(3), skel, &[]);
                            ui.end_row();

                            ui.label("Unk Type");
                            egui::ComboBox::from_id_source(id.with(4))
                                .selected_text(o.unk_type.to_string())
                                .show_ui(ui, |ui| {
                                    changed |=
                                        ui.selectable_value(&mut o.unk_type, 1, "1").changed();
                                    changed |=
                                        ui.selectable_value(&mut o.unk_type, 2, "2").changed();
                                });
                            ui.end_row();

                            ui.label("Constraint Axes");
                            changed |= edit_vector3(ui, id.with(5), &mut o.constraint_axes);
                            ui.end_row();

                            ui.label("Quat 1");
                            changed |= edit_vector4(ui, id.with(6), &mut o.quat1);
                            ui.end_row();

                            ui.label("Quat 2");
                            changed |= edit_vector4(ui, id.with(7), &mut o.quat2);
                            ui.end_row();

                            ui.label("Range Min");
                            changed |= edit_vector3(ui, id.with(8), &mut o.range_min);
                            ui.end_row();

                            ui.label("Range Max");
                            changed |= edit_vector3(ui, id.with(9), &mut o.range_max);
                            ui.end_row();
                        });
                    });
            }
        });
    changed
}

fn aim_constraints(ui: &mut Ui, hlpb: &mut HlpbData, skel: Option<&SkelData>) -> bool {
    let mut changed = false;
    CollapsingHeader::new("Aim Constraints")
        .default_open(true)
        .show(ui, |ui| {
            for (i, aim) in hlpb.aim_constraints.iter_mut().enumerate() {
                let id = egui::Id::new("aim").with(i);

                // Append the helper bone names to make it easier to find constraints.
                CollapsingHeader::new(format!(
                    "{} ({} / {})",
                    aim.name, aim.target_bone_name1, aim.target_bone_name2
                ))
                .id_source(id.with(&aim.name))
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new(id).show(ui, |ui| {
                        ui.label("Name");
                        changed |= ui.text_edit_singleline(&mut aim.name).changed();
                        ui.end_row();

                        ui.label("Aim 1");
                        changed |=
                            bone_combo_box(ui, &mut aim.aim_bone_name1, id.with(0), skel, &[]);
                        ui.end_row();

                        ui.label("Aim 2");
                        changed |=
                            bone_combo_box(ui, &mut aim.aim_bone_name2, id.with(1), skel, &[]);
                        ui.end_row();

                        ui.label("Aim Type 1");
                        changed |=
                            bone_combo_box(ui, &mut aim.aim_type1, id.with(2), skel, &["DEFAULT"]);
                        ui.end_row();

                        ui.label("Aim Type 2");
                        changed |=
                            bone_combo_box(ui, &mut aim.aim_type2, id.with(3), skel, &["DEFAULT"]);
                        ui.end_row();

                        ui.label("Target 1");
                        changed |=
                            bone_combo_box(ui, &mut aim.target_bone_name1, id.with(4), skel, &[]);
                        ui.end_row();

                        ui.label("Target 2");
                        changed |=
                            bone_combo_box(ui, &mut aim.target_bone_name2, id.with(5), skel, &[]);
                        ui.end_row();

                        ui.label("Unk1");
                        changed |= ui.add(DragValue::new(&mut aim.unk1)).changed();
                        ui.end_row();

                        ui.label("Unk2");
                        changed |= ui.add(DragValue::new(&mut aim.unk2)).changed();
                        ui.end_row();

                        ui.label("Aim");
                        changed |= edit_vector3(ui, id.with(6), &mut aim.aim);
                        ui.end_row();

                        ui.label("Up");
                        changed |= edit_vector3(ui, id.with(7), &mut aim.up);
                        ui.end_row();

                        ui.label("Quat 1");
                        changed |= edit_vector4(ui, id.with(8), &mut aim.quat1);
                        ui.end_row();

                        ui.label("Quat 2");
                        changed |= edit_vector4(ui, id.with(9), &mut aim.quat2);
                        ui.end_row();
                    });
                });
            }
        });
    changed
}

fn edit_vector3(ui: &mut Ui, id: egui::Id, value: &mut Vector3) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui
            .add(DragSlider::new(id.with("x"), &mut value.x).width(40.0))
            .changed();
        changed |= ui
            .add(DragSlider::new(id.with("y"), &mut value.y).width(40.0))
            .changed();
        changed |= ui
            .add(DragSlider::new(id.with("z"), &mut value.z).width(40.0))
            .changed();
    });
    changed
}

fn edit_vector4(ui: &mut Ui, id: egui::Id, value: &mut Vector4) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui
            .add(DragSlider::new(id.with("x"), &mut value.x).width(40.0))
            .changed();
        changed |= ui
            .add(DragSlider::new(id.with("y"), &mut value.y).width(40.0))
            .changed();
        changed |= ui
            .add(DragSlider::new(id.with("z"), &mut value.z).width(40.0))
            .changed();
        changed |= ui
            .add(DragSlider::new(id.with("w"), &mut value.w).width(40.0))
            .changed();
    });
    changed
}
