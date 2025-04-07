use bevy::ecs::query;
use bevy::{ prelude::*, ui::update };
use transform_gizmo_bevy::{ config::TransformPivotPoint, gizmo, prelude::* };
use crate::web::get_tool_type;
use crate::models::{ ToolType, SelectedTool };
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_gizmo);
        app.insert_resource(SelectedTool::default());
    }
}
fn update_gizmo(
    mut gizmo_options: ResMut<GizmoOptions>,
    gizmo_targets: Query<&GizmoTarget>,
    mut selected_tool: ResMut<SelectedTool>
) {
    let all_modes = &mut gizmo_options.gizmo_modes;
    let tool_type: ToolType = get_tool_type().into();
    // info!("tool_type: {:?}", get_tool_type().to_string());
    match tool_type {
        ToolType::Transform => {
            // if selected_tool.is_move_enabled {
            //     all_modes.remove(GizmoMode::TranslateView);
            //     all_modes.remove(GizmoMode::TranslateX);
            //     all_modes.remove(GizmoMode::TranslateY);
            //     all_modes.remove(GizmoMode::TranslateZ);
            //     // all_modes.remove(GizmoMode::Arcball);
            //     selected_tool.is_move_enabled = false;
            // }
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::TranslateView);
            all_modes.insert(GizmoMode::RotateView);
            all_modes.insert(GizmoMode::ScaleUniform);
            all_modes.insert(GizmoMode::ScaleXZ);
            all_modes.insert(GizmoMode::ScaleXY);
            all_modes.insert(GizmoMode::ScaleYZ);
            all_modes.insert(GizmoMode::ScaleX);
            all_modes.insert(GizmoMode::ScaleY);
            all_modes.insert(GizmoMode::ScaleZ);
            all_modes.insert(GizmoMode::TranslateX);
            all_modes.insert(GizmoMode::TranslateY);
            all_modes.insert(GizmoMode::TranslateZ);
            all_modes.insert(GizmoMode::TranslateXY);
            all_modes.insert(GizmoMode::TranslateXZ);
            all_modes.insert(GizmoMode::TranslateYZ);
            all_modes.insert(GizmoMode::RotateX);
            all_modes.insert(GizmoMode::RotateY);
            all_modes.insert(GizmoMode::RotateZ);
        }
        ToolType::Select => {}
        ToolType::Drag => {
            all_modes.insert(GizmoMode::TranslateView);
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::RotateView);
            all_modes.remove(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::ScaleXZ);
            all_modes.remove(GizmoMode::ScaleXY);
            all_modes.remove(GizmoMode::ScaleYZ);
            all_modes.remove(GizmoMode::ScaleX);
            all_modes.remove(GizmoMode::ScaleY);
            all_modes.remove(GizmoMode::ScaleZ);
            all_modes.remove(GizmoMode::TranslateX);
            all_modes.remove(GizmoMode::TranslateY);
            all_modes.remove(GizmoMode::TranslateZ);
            all_modes.remove(GizmoMode::TranslateXY);
            all_modes.remove(GizmoMode::TranslateXZ);
            all_modes.remove(GizmoMode::TranslateYZ);
            all_modes.remove(GizmoMode::RotateX);
            all_modes.remove(GizmoMode::RotateY);
            all_modes.remove(GizmoMode::RotateZ);
        }
        ToolType::Move => {
            if !selected_tool.is_move_enabled {
                selected_tool.is_move_enabled = true;
            }
            all_modes.insert(GizmoMode::TranslateX);
            all_modes.insert(GizmoMode::TranslateY);
            all_modes.insert(GizmoMode::TranslateZ);
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::RotateView);
            all_modes.remove(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::TranslateView);
            all_modes.remove(GizmoMode::ScaleXZ);
            all_modes.remove(GizmoMode::ScaleXY);
            all_modes.remove(GizmoMode::ScaleYZ);
            all_modes.remove(GizmoMode::ScaleX);
            all_modes.remove(GizmoMode::ScaleY);
            all_modes.remove(GizmoMode::ScaleZ);
            all_modes.remove(GizmoMode::TranslateXY);
            all_modes.remove(GizmoMode::TranslateXZ);
            all_modes.remove(GizmoMode::TranslateYZ);
            all_modes.remove(GizmoMode::RotateX);
            all_modes.remove(GizmoMode::RotateY);
            all_modes.remove(GizmoMode::RotateZ);
        }
        ToolType::Rotate => {
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::RotateView);
            all_modes.remove(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::TranslateView);
            all_modes.remove(GizmoMode::ScaleXZ);
            all_modes.remove(GizmoMode::ScaleXY);
            all_modes.remove(GizmoMode::ScaleYZ);
            all_modes.remove(GizmoMode::ScaleX);
            all_modes.remove(GizmoMode::ScaleY);
            all_modes.remove(GizmoMode::ScaleZ);
            all_modes.remove(GizmoMode::TranslateX);
            all_modes.remove(GizmoMode::TranslateY);
            all_modes.remove(GizmoMode::TranslateZ);
            all_modes.remove(GizmoMode::TranslateXY);
            all_modes.remove(GizmoMode::TranslateXZ);
            all_modes.remove(GizmoMode::TranslateYZ);
            all_modes.insert(GizmoMode::RotateX);
            all_modes.insert(GizmoMode::RotateY);
            all_modes.insert(GizmoMode::RotateZ);
        }
        ToolType::Scale => {
            // all_modes.insert(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::RotateView);
            all_modes.remove(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::TranslateView);
            all_modes.remove(GizmoMode::ScaleXZ);
            all_modes.remove(GizmoMode::ScaleXY);
            all_modes.remove(GizmoMode::ScaleYZ);
            all_modes.remove(GizmoMode::TranslateX);
            all_modes.remove(GizmoMode::TranslateY);
            all_modes.remove(GizmoMode::TranslateZ);
            all_modes.remove(GizmoMode::TranslateXY);
            all_modes.remove(GizmoMode::TranslateXZ);
            all_modes.remove(GizmoMode::TranslateYZ);
            all_modes.remove(GizmoMode::RotateX);
            all_modes.remove(GizmoMode::RotateY);
            all_modes.remove(GizmoMode::RotateZ);
            all_modes.insert(GizmoMode::ScaleX);
            all_modes.insert(GizmoMode::ScaleY);
            all_modes.insert(GizmoMode::ScaleZ);
        }
        ToolType::None => {
            all_modes.remove(GizmoMode::Arcball);
            all_modes.remove(GizmoMode::RotateView);
            all_modes.remove(GizmoMode::ScaleUniform);
            all_modes.remove(GizmoMode::TranslateView);
            all_modes.remove(GizmoMode::ScaleXZ);
            all_modes.remove(GizmoMode::ScaleXY);
            all_modes.remove(GizmoMode::ScaleYZ);
            all_modes.remove(GizmoMode::ScaleX);
            all_modes.remove(GizmoMode::ScaleY);
            all_modes.remove(GizmoMode::ScaleZ);
            all_modes.remove(GizmoMode::TranslateX);
            all_modes.remove(GizmoMode::TranslateY);
            all_modes.remove(GizmoMode::TranslateZ);
            all_modes.remove(GizmoMode::TranslateXY);
            all_modes.remove(GizmoMode::TranslateXZ);
            all_modes.remove(GizmoMode::TranslateYZ);
            all_modes.remove(GizmoMode::RotateX);
            all_modes.remove(GizmoMode::RotateY);
            all_modes.remove(GizmoMode::RotateZ);
        }
    }
}
