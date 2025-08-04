use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.init_resource::<FlyCameraSettings>();

        app.add_systems(Update, camera_look);
        app.add_systems(Update, camera_move);
    }
}

#[derive(Resource)]
pub struct FlyCameraSettings {
    pub move_speed: f32,
    pub look_speed: f32,
}

impl Default for FlyCameraSettings {
    fn default() -> Self {
        Self { move_speed: 10., look_speed: 0.001 }
    }
}

#[derive(Component)]
pub struct FlyCamera;

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        FlyCamera,
        Camera3d::default(),
        Transform::from_xyz(0., 0., 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_look(
    mut query: Query<&mut Transform, With<FlyCamera>>,
    settings: Res<FlyCameraSettings>,
    mouse: Res<AccumulatedMouseMotion>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.single_inner()
    else {
        return;
    };

    let delta = mouse.delta;
    let (dpitch, dyaw) = (
        -delta.y * settings.look_speed * window.scale_factor(),
        -delta.x * settings.look_speed * window.scale_factor(),
    );

    for mut transform in &mut query {
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw + dyaw,
            (pitch + dpitch).to_degrees().clamp(-89., 89.).to_radians(),
            0.,
        );
    }
}

fn camera_move(
    mut query: Query<&mut Transform, With<FlyCamera>>,
    settings: Res<FlyCameraSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let mut accumulated_movement = Vec3::ZERO;
        let (f, r, u) = (transform.forward(), transform.right(), transform.up());
        for key in keys.get_pressed() {
            match key {
                | KeyCode::KeyW => accumulated_movement += f.as_vec3(),
                | KeyCode::KeyS => accumulated_movement -= f.as_vec3(),
                | KeyCode::KeyD => accumulated_movement += r.as_vec3(),
                | KeyCode::KeyA => accumulated_movement -= r.as_vec3(),
                | KeyCode::KeyR => accumulated_movement += u.as_vec3(),
                | KeyCode::KeyF => accumulated_movement -= u.as_vec3(),
                | _ => {}
            }
        }

        let movement = accumulated_movement.normalize_or_zero();
        transform.translation += movement * settings.move_speed * time.delta_secs();
    }
}
