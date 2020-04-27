use amethyst::{
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        Transform,
    },
    derive::SystemDesc,
    //ecs::World,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
    winit::MouseButton,
};

#[derive(SystemDesc)]
pub struct CameraSystem {
    last_mouse_pos: (f32, f32),
    free_cam_mode: bool,
    //mouse_button_down: bool,
}

impl CameraSystem {
    pub fn new() -> Self {
        CameraSystem {
            last_mouse_pos: (0.0, 0.0),
            free_cam_mode: false,
            //mouse_button_down: false,
        }
    }
}

impl<'s> System<'s> for CameraSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    // fn setup(&mut self, world: &mut World) {
    //     println!("setup system");
    // }

    fn run(&mut self, (mut transforms, cameras, input, screen_dimensions): Self::SystemData) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            let switch_mode = input.action_is_down("switch_mode").unwrap_or(false);
            if switch_mode {
                self.free_cam_mode = !self.free_cam_mode;
                println!("switch mode to free cam? {}", self.free_cam_mode);
            }

            // Gets mouse coordinates
            if let Some((x, y)) = input.mouse_position() {
                if self.last_mouse_pos.0 == 0.0 && self.last_mouse_pos.1 == 0.0 {
                    self.last_mouse_pos = (x, y);
                }
                let delta_mouse_pos = (x - self.last_mouse_pos.0, y - self.last_mouse_pos.1);
                self.last_mouse_pos = (x, y);

                if self.free_cam_mode {
                    let pos = *transform.translation();
                    transform.set_translation_xyz(0.0, 0.0, 0.0);

                    // println!("mouse delta: {:?}", delta_mouse_pos);

                    transform.append_rotation_x_axis(-0.3 * delta_mouse_pos.1.to_radians());
                    transform.append_rotation_y_axis(-0.3 * delta_mouse_pos.0.to_radians());

                    transform.prepend_translation(pos);

                    let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
                    if horizontal != 0.0 {
                        transform.move_right(0.5 * horizontal);
                    }
                    let vertical = input.axis_value("vertical").unwrap_or(0.0);
                    if vertical != 0.0 {
                        transform.move_forward(0.5 * vertical);
                    }
                } else {
                    // panning + zoom
                    let wheel = input.mouse_wheel_value(false);
                    // wheel value is -1, 0, or 1
                    if wheel != 0.0 {
                        //transform.move_forward(wheel);
                        let ray = camera.projection().screen_ray(
                            Point2::new(x, y),
                            Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                            transform,
                        );
                        let distance = ray.intersect_plane(&Plane::with_y(0.0)).unwrap();
                        let mouse_map_position = ray.at_distance(distance);

                        let direction = Vector3::new(
                            mouse_map_position.x,
                            mouse_map_position.y,
                            mouse_map_position.z,
                        ) - transform.translation();
                        transform.prepend_translation(direction * 0.1 * wheel);
                    }

                    if input.mouse_button_is_down(MouseButton::Middle) {
                        transform.move_right(delta_mouse_pos.0 * -0.01);
                        transform.move_up(delta_mouse_pos.1 * 0.01);
                    }
                }
            }
        }
    }
}
