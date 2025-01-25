use bevy::{
    //color::palettes::tailwind, 
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin}, 
    //ecs::component::{self, ComponentHooks, StorageType}, 
    math::{uvec2, vec2, vec3}, 
    prelude::*, 
    reflect::TypePath, 
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, 
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}
};



use std::sync::{Arc, Mutex};
use bevy_flycam::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use turborand::prelude::*;
//use std::time::Instant;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    //present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<RaymarchMaterial>::default(),
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        // Here we define size of our overlay
                        font_size: 30.0,
                        // We can also change color of the overlay
                        color: Color::srgb(0.5, 0.5, 0.5),
                        // If we want, we can use a custom font
                        font: default(),
                    },
                },
            },
            NoCameraPlayerPlugin,
        ))
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        .register_type::<(SdCube, RayCamera, SdSphere, RaymarchSettings, SdDirectionalLight, SdEllipse, SdTorus, SdCylinder, SdCone)>()

        
        .insert_resource(ShapeContainer::default())
        .insert_resource(BvhTree::default())
        .insert_resource(RaymarchSettings{
            max_distance: 1000.0, 
            lower_colour: vec3(0.1, 0.1, 0.2), 
            middle_colour: vec3(0.1, 0.1, 0.4), 
            upper_colour: vec3(0.4, 0.5, 0.75), 
            skybox_powers: vec2(2.0, 0.6), 
            shadow_power: 0.005,
        })
        .add_systems(Startup, register_sdf_hooks.before(setup))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, (push_shapes, window_resize).before(set_mat_values))
        .add_systems(PostUpdate, set_mat_values)
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ControlLeft,
            toggle_grab_cursor: KeyCode::KeyZ,
            ..Default::default()
        })
        //.init_state::<ScrollType>()
        .run();
}

fn register_sdf_hooks(world: &mut World) {
    world.register_component_hooks::<SdSphere>().on_remove(|world, entity, _component_id|{
        let sphere = world.get::<SdSphere>(entity).unwrap();
        let sphere_index = sphere.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(1, sphere_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.spheres.lock().unwrap().swap_remove(sphere.index as usize);
        if sphere_index as usize != container.spheres.lock().unwrap().len() - 1 {
            let replacement = container.spheres.lock().unwrap()[sphere.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(1, sphere_index),
                _ => panic!("resetting a swapped sphere parent failed since the type was not known"),
            }
        }
    });
    world.register_component_hooks::<SdCube>().on_remove(|world, entity, _component_id|{
        println!("removing cube");
        let cube = world.get::<SdCube>(entity).unwrap();
        let cube_index = cube.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(2, cube_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.cubes.lock().unwrap().swap_remove(cube.index as usize);
        if cube_index as usize != container.cubes.lock().unwrap().len() - 1 {
            let replacement = container.cubes.lock().unwrap()[cube.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(2, cube_index),
                _ => panic!("resetting a swapped cube parent failed since the type was not known"),
            }
        }
    });
    world.register_component_hooks::<SdEllipse>().on_remove(|world, entity, _component_id|{
        println!("removing ellipse");
        let ellipse = world.get::<SdEllipse>(entity).unwrap();
        let ellipse_index = ellipse.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(3, ellipse_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.ellipses.lock().unwrap().swap_remove(ellipse.index as usize);
        if ellipse_index as usize != container.ellipses.lock().unwrap().len() - 1 {
            let replacement = container.ellipses.lock().unwrap()[ellipse.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(3, ellipse_index),
                _ => panic!("resetting a swapped ellipse parent failed since the type was not known"),
            }
        }
    });
    world.register_component_hooks::<SdTorus>().on_remove(|world, entity, _component_id|{
        println!("removing torus");
        let torus = world.get::<SdTorus>(entity).unwrap();
        let torus_index = torus.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(4, torus_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.toruses.lock().unwrap().swap_remove(torus.index as usize);
        if torus_index as usize != container.toruses.lock().unwrap().len() - 1 {
            let replacement = container.toruses.lock().unwrap()[torus.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(4, torus_index),
                _ => panic!("resetting a swapped torus parent failed since the type was not known"),
            }
        }
    });
    world.register_component_hooks::<SdCylinder>().on_remove(|world, entity, _component_id|{
        println!("removing cylinder");
        let cylinder = world.get::<SdCylinder>(entity).unwrap();
        let cylinder_index = cylinder.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(5, cylinder_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.cylinders.lock().unwrap().swap_remove(cylinder.index as usize);
        if cylinder_index as usize != container.cylinders.lock().unwrap().len() - 1 {
            let replacement = container.cylinders.lock().unwrap()[cylinder.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(5, cylinder_index),
                _ => panic!("resetting a swapped cylinder parent failed since the type was not known"),
            }
        }
    });
    world.register_component_hooks::<SdCone>().on_remove(|world, entity, _component_id|{
        println!("removing cone");
        let cone = world.get::<SdCone>(entity).unwrap();
        let cone_index = cone.index;
        let tree = world.resource::<BvhTree>();
        let container = world.resource::<ShapeContainer>();
        remove_leaf(uvec2(6, cone_index), container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));
        container.cones.lock().unwrap().swap_remove(cone.index as usize);
        if cone_index as usize != container.cones.lock().unwrap().len() - 1 {
            let replacement = container.cones.lock().unwrap()[cone.index as usize];
            let parent = replacement.parent_idx;
            match parent.x{
                0 => tree.nodes.lock().unwrap()[parent.y as usize].child1 = uvec2(5, cone_index),
                _ => panic!("resetting a swapped cone parent failed since the type was not known"),
            }
        }
    });

}


// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RaymarchMaterial>>,
) {

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(RaymarchMaterial {
            position: Vec3::new(1.0, 0.0, 0.0),
            forward: Vec3::new(0.0, 0.0, -1.0),
            horizontal: Vec3::new(1.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 1.0, 0.0),
            fov: 90.0,
            root_index: 1,
            raymarch_settings: RaymarchSettings{max_distance: 1000.0, ..Default::default() },
            nodes: vec![BvhNode::default()],
            dir_lights: vec![],
            pos_lights: vec![],
            spheres: vec![],
            cubes: vec![],
            ellipses: vec![],
            toruses: vec![],
            cylinders: vec![],
            cones: vec![],
        }),
        ..default()
    }).insert((RayImage, Name::new("Quad"),));


    // camera
    commands.spawn(Camera2dBundle::default());

    //raycam

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(2.0, 1.5, -10.0).looking_at(Vec3::splat(0.0), Vec3::Y),
            ..Default::default()
        }, RayCamera{fov: 90.0,},
        FlyCam,
        Name::new("Camera"),
    ));

    //mainlight

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }, SdDirectionalLight{strength: 1.0, colour: vec3(0.8, 0.75, 0.8), ..Default::default()},
        Name::new("Directional Light"),
    ));
 

    



    let a_rand = AtomicRng::with_seed(2);

    let bounds = 30.0;

    let count: usize = 20;

    
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }, SdCube{size: vec3(bounds, 0.1, bounds), colour: Vec3::splat(a_rand.f32()), ..Default::default()},
        Name::new("Ground"),
    ));


    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdSphere{radius: a_rand.f32() + 0.1,colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()) ,..Default::default()},
            Name::new("Sphere"),
        ));
    }


    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdCube{size: Vec3::new(a_rand.f32() + 0.1, a_rand.f32() + 0.1, a_rand.f32() + 0.1,), colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()), ..Default::default()},
            Name::new("Cube"),
        ));
    }

    

    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdEllipse{radii: Vec3::new(a_rand.f32() + 0.1, a_rand.f32() + 0.1, a_rand.f32() + 0.1,), colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()), ..Default::default()},
            Name::new("Ellipse"),
        ));
    }
    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdTorus{radii: Vec2::new(a_rand.f32() + 0.1, a_rand.f32() + 0.1,), colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()), ..Default::default()},
            Name::new("Torus"),
        ));
    }
    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdCylinder{height: a_rand.f32() + 0.1, radius: a_rand.f32() + 0.1, colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()), ..Default::default()},
            Name::new("Cylinder"),
        ));
    }
    for _i in 0..count{
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(a_rand.f32_normalized() * bounds, a_rand.f32() * bounds + 1.0, a_rand.f32_normalized() * bounds),
                ..Default::default()
            }, SdCone{height: a_rand.f32() + 0.1, sincos: vec2(a_rand.f32() + 0.1, a_rand.f32() + 0.1), colour: vec3(a_rand.f32(), a_rand.f32(), a_rand.f32()), ..Default::default()},
            Name::new("Cone"),
        ));
    }
}



// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RaymarchMaterial {
    #[uniform(1)]
    position: Vec3,
    #[uniform(2)]
    forward: Vec3,
    #[uniform(3)]
    horizontal: Vec3,
    #[uniform(4)]
    vertical: Vec3,
    #[uniform(5)]
    fov: f32,
    #[uniform(6)]
    root_index: u32,
    #[uniform(7)]
    raymarch_settings: RaymarchSettings,
    #[storage(8, read_only)]
    nodes: Vec<BvhNode>,
    #[storage(9, read_only)]
    dir_lights: Vec<SdDirectionalLight>,
    #[storage(10, read_only)]
    pos_lights: Vec<SdPositionalLight>,
    #[storage(11, read_only)]
    spheres: Vec<SdSphere>,
    #[storage(12, read_only)]
    cubes: Vec<SdCube>,
    #[storage(13, read_only)]
    ellipses: Vec<SdEllipse>,
    #[storage(14, read_only)]
    toruses: Vec<SdTorus>,
    #[storage(15, read_only)]
    cylinders: Vec<SdCylinder>,
    #[storage(16, read_only)]
    cones: Vec<SdCone>,

}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for RaymarchMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/raymarch.wgsl".into()
    }
}

#[derive(Component)]
struct RayImage;

#[derive(Component, Reflect)]
pub struct RayCamera{
    pub fov: f32,
}

fn window_resize(
    windows: Query<&Window, Changed<Window>>,
    mut ray_t_q: Query<&mut Transform, With<RayImage>>,
){
    
    if !windows.is_empty() {

    let window = windows.get_single().unwrap();

    let mut ray_t = ray_t_q.get_single_mut().unwrap();

    ray_t.scale = Vec3::new(window.width(), window.height(), 1.0);

    }
}

fn set_mat_values(
    mut rayt: Query<(&GlobalTransform, &mut RayCamera)>,
    mut materials: ResMut<Assets<RaymarchMaterial>>,
    march_handle_q: Query<&mut Handle<RaymarchMaterial>, With<RayImage>>,
    shapes_res: Res<ShapeContainer>,
    tree_res: Res<BvhTree>,
    settings_res: Res<RaymarchSettings>,
    mut dir_light_q: Query<(&mut SdDirectionalLight, &GlobalTransform)>,
    mut pos_light_q: Query<(&mut SdPositionalLight, &GlobalTransform)>,
){
    
    let mut dir_light_vec: Vec<SdDirectionalLight> = vec![];
    let mut pos_light_vec: Vec<SdPositionalLight> = vec![];

    //do light stuff straight in the mat values
    
    for (mut light, gt) in &mut dir_light_q {
        light.direction = Dir3::as_vec3(&gt.up());
        dir_light_vec.push(*light);

    }
    
    for (mut light, gt) in &mut pos_light_q {
        light.translation = gt.translation();
        pos_light_vec.push(*light);
    }

    let (transform, raycam) = rayt.get_single_mut().unwrap();

    let march_handle = march_handle_q.get_single().unwrap().id();

    if let Some(material) =  materials.get_mut(march_handle){
        material.position = transform.translation();
        material.forward = Dir3::as_vec3(&transform.forward());
        material.horizontal = Dir3::as_vec3(&transform.right());
        material.vertical = Dir3::as_vec3(&transform.up());
        material.fov = raycam.fov;
        material.root_index = *tree_res.root_index.lock().unwrap();
        material.dir_lights = dir_light_vec;
        material.pos_lights = pos_light_vec;
        material.nodes = tree_res.nodes.lock().unwrap().clone();
        material.raymarch_settings = *settings_res;
        material.spheres = shapes_res.spheres.lock().unwrap().clone();
        material.cubes = shapes_res.cubes.lock().unwrap().clone();
        material.ellipses = shapes_res.ellipses.lock().unwrap().clone();
        material.toruses = shapes_res.toruses.lock().unwrap().clone();
        material.cylinders = shapes_res.cylinders.lock().unwrap().clone();
        material.cones = shapes_res.cones.lock().unwrap().clone();
    }
}



fn push_shapes(
    container: ResMut<ShapeContainer>,
    tree: ResMut<BvhTree>,
    mut spheres: ParamSet<(
        Query<(&mut SdSphere, &GlobalTransform), Added<SdSphere>>,
        Query<(&mut SdSphere, &GlobalTransform), Or<(Changed<SdSphere>, Changed<GlobalTransform>)>>,
    )>,
    mut cubes: ParamSet<(
        Query<(&mut SdCube, &GlobalTransform), Added<SdCube>>,   
        Query<(&mut SdCube, &GlobalTransform), Or<(Changed<SdCube>, Changed<GlobalTransform>)>>,
    )>,
    mut ellipses: ParamSet<(
        Query<(&mut SdEllipse, &GlobalTransform), Added<SdEllipse>>,   
        Query<(&mut SdEllipse, &GlobalTransform), Or<(Changed<SdEllipse>, Changed<GlobalTransform>)>>,
    )>,
    mut toruses: ParamSet<(
        Query<(&mut SdTorus, &GlobalTransform), Added<SdTorus>>,   
        Query<(&mut SdTorus, &GlobalTransform), Or<(Changed<SdTorus>, Changed<GlobalTransform>)>>,
    )>,
    mut cylinders: ParamSet<(
        Query<(&mut SdCylinder, &GlobalTransform), Added<SdCylinder>>,   
        Query<(&mut SdCylinder, &GlobalTransform), Or<(Changed<SdCylinder>, Changed<GlobalTransform>)>>,
    )>,
    mut cones: ParamSet<(
        Query<(&mut SdCone, &GlobalTransform), Added<SdCone>>,   
        Query<(&mut SdCone, &GlobalTransform), Or<(Changed<SdCone>, Changed<GlobalTransform>)>>,
    )>,

){


    //let pushing = Instant::now();

    spheres.p0().par_iter_mut().for_each(|(mut sphere, gt)| {
        let transform = gt.compute_matrix();
        sphere.transform_determinant = transform.determinant();
        sphere.inverse_transform = transform.inverse();

        let mut spheres = container.spheres.lock().unwrap();
        sphere.index = spheres.len() as u32;
        spheres.push(*sphere);
        drop(spheres);

        let shape_idx = uvec2(1, sphere.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        sphere.parent_idx = container.spheres.lock().unwrap()[sphere.index as usize].parent_idx;

    });

    

    spheres.p1().par_iter_mut().for_each(|(mut sphere, gt)| { 
        let transform = gt.compute_matrix();
        sphere.transform_determinant = transform.determinant();
        sphere.inverse_transform = transform.inverse();
        container.spheres.lock().unwrap()[sphere.index as usize] = *sphere;
        let shape_idx = uvec2(1, sphere.index);
        

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        sphere.parent_idx = container.spheres.lock().unwrap()[sphere.index as usize].parent_idx;
    });



    cubes.p0().par_iter_mut().for_each(|(mut ellipse, gt)| {
        let transform = gt.compute_matrix();
        ellipse.transform_determinant = transform.determinant();
        ellipse.inverse_transform = transform.inverse();
        let mut cubes = container.cubes.lock().unwrap();
        ellipse.index = cubes.len() as u32;
        cubes.push(*ellipse);
        drop(cubes);

        let shape_idx = uvec2(2, ellipse.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        ellipse.parent_idx = container.cubes.lock().unwrap()[ellipse.index as usize].parent_idx;
    });



    cubes.p1().par_iter_mut().for_each(|(mut cube, gt)| { 
        let transform = gt.compute_matrix();
        cube.transform_determinant = transform.determinant();
        cube.inverse_transform = transform.inverse();
        container.cubes.lock().unwrap()[cube.index as usize] = *cube;
        let shape_idx = uvec2(2, cube.index);

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        cube.parent_idx = container.cubes.lock().unwrap()[cube.index as usize].parent_idx;
    });


    ellipses.p0().par_iter_mut().for_each(|(mut ellipse, gt)| {
        let transform = gt.compute_matrix();
        ellipse.transform_determinant = transform.determinant();
        ellipse.inverse_transform = transform.inverse();
        let mut ellipses = container.ellipses.lock().unwrap();
        ellipse.index = ellipses.len() as u32;
        ellipses.push(*ellipse);
        drop(ellipses);

        let shape_idx = uvec2(3, ellipse.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        ellipse.parent_idx = container.ellipses.lock().unwrap()[ellipse.index as usize].parent_idx;
    });

    ellipses.p1().par_iter_mut().for_each(|(mut ellipse, gt)| { 
        let transform = gt.compute_matrix();
        ellipse.transform_determinant = transform.determinant();
        ellipse.inverse_transform = transform.inverse();
        container.ellipses.lock().unwrap()[ellipse.index as usize] = *ellipse;
        let shape_idx = uvec2(3, ellipse.index);

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        ellipse.parent_idx = container.ellipses.lock().unwrap()[ellipse.index as usize].parent_idx;
    });

    toruses.p0().par_iter_mut().for_each(|(mut torus, gt)| {
        let transform = gt.compute_matrix();
        torus.transform_determinant = transform.determinant();
        torus.inverse_transform = transform.inverse();
        let mut toruses = container.toruses.lock().unwrap();
        torus.index = toruses.len() as u32;
        toruses.push(*torus);
        drop(toruses);

        let shape_idx = uvec2(4, torus.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        torus.parent_idx = container.toruses.lock().unwrap()[torus.index as usize].parent_idx;
    });
    toruses.p1().par_iter_mut().for_each(|(mut torus, gt)| { 
        let transform = gt.compute_matrix();
        torus.transform_determinant = transform.determinant();
        torus.inverse_transform = transform.inverse();
        container.toruses.lock().unwrap()[torus.index as usize] = *torus;
        let shape_idx = uvec2(4, torus.index);

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        torus.parent_idx = container.toruses.lock().unwrap()[torus.index as usize].parent_idx;
    });
    cylinders.p0().par_iter_mut().for_each(|(mut cylinder, gt)| {
        let transform = gt.compute_matrix();
        cylinder.transform_determinant = transform.determinant();
        cylinder.inverse_transform = transform.inverse();
        let mut cylinders = container.cylinders.lock().unwrap();
        cylinder.index = cylinders.len() as u32;
        cylinders.push(*cylinder);
        drop(cylinders);

        let shape_idx = uvec2(5, cylinder.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        cylinder.parent_idx = container.cylinders.lock().unwrap()[cylinder.index as usize].parent_idx;
    });
    cylinders.p1().par_iter_mut().for_each(|(mut cylinder, gt)| { 
        let transform = gt.compute_matrix();
        cylinder.transform_determinant = transform.determinant();
        cylinder.inverse_transform = transform.inverse();
        container.cylinders.lock().unwrap()[cylinder.index as usize] = *cylinder;
        let shape_idx = uvec2(5, cylinder.index);

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        cylinder.parent_idx = container.cylinders.lock().unwrap()[cylinder.index as usize].parent_idx;
    });

    //println!("building took {:?} milliseconds", pushing.elapsed().as_secs_f64() * 1000.0);
    cones.p0().par_iter_mut().for_each(|(mut cone, gt)| {
        let transform = gt.compute_matrix();
        cone.transform_determinant = transform.determinant();
        cone.inverse_transform = transform.inverse();
        let mut cones = container.cones.lock().unwrap();
        cone.index = cones.len() as u32;
        cones.push(*cone);
        drop(cones);

        let shape_idx = uvec2(6, cone.index);

        insert_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.node_count), Arc::clone(&tree.root_index));

        cone.parent_idx = container.cones.lock().unwrap()[cone.index as usize].parent_idx;
    });
    cones.p1().par_iter_mut().for_each(|(mut cone, gt)| { 
        let transform = gt.compute_matrix();
        cone.transform_determinant = transform.determinant();
        cone.inverse_transform = transform.inverse();
        container.cones.lock().unwrap()[cone.index as usize] = *cone;
        let shape_idx = uvec2(6, cone.index);

        refit_leaf(shape_idx, &container, Arc::clone(&tree.nodes), Arc::clone(&tree.root_index));
        cone.parent_idx = container.cones.lock().unwrap()[cone.index as usize].parent_idx;
    });

}



//structs

#[derive(Debug, Clone, Copy, ShaderType)]
struct Aabb{
    min: Vec3,
    max: Vec3,
}

impl Default for Aabb {
    fn default() -> Self { 
        Self{
            min: Vec3::MAX, 
            max: Vec3::MIN
        } }
}

#[derive(Default, Debug, Clone, Copy)]
struct Obb{
    center: Vec3,
    size: Vec3,
    rotation: Quat,
}

#[derive(Default, Debug, Clone, Copy, ShaderType)]
struct BvhNode{
    o_p_idx: UVec2,
    aabb: Aabb,
    child1: UVec2,
    child2: UVec2,
}

#[derive(Resource, Default, Debug, Clone)]
struct BvhTree{
    node_count: Arc<Mutex<u32>>,
    root_index: Arc<Mutex<u32>>,
    nodes: Arc<Mutex<Vec<BvhNode>>>
}

#[derive(Resource, Default, Debug, Clone)]
struct ShapeContainer{
    spheres: Arc<Mutex<Vec<SdSphere>>>,
    cubes: Arc<Mutex<Vec<SdCube>>>,
    ellipses: Arc<Mutex<Vec<SdEllipse>>>,
    toruses: Arc<Mutex<Vec<SdTorus>>>,
    cylinders: Arc<Mutex<Vec<SdCylinder>>>,
    cones: Arc<Mutex<Vec<SdCone>>>,
}


#[derive(Resource, Default, Debug, Clone, Copy, ShaderType, Reflect)]
#[reflect(Resource)]
pub struct RaymarchSettings{
    pub lower_colour: Vec3,
    pub middle_colour: Vec3,
    pub upper_colour: Vec3,
    pub max_distance: f32,
    pub skybox_powers: Vec2,
    pub shadow_power: f32,
}


// Uvec2 so we can represent multiple different objects:
// when ids[i].x = 0, we are referencing the (ids[i].y)th element in the vec of nodes
// [0, 3] would be the 4th element in the node array
// [1, 8] would be the 9th element in the sphere array
// [2, 0] would be the 1st element in the cube array

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdDirectionalLight{
    pub strength: f32,
    pub colour: Vec3,
    direction: Vec3,
    
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdPositionalLight{
    pub strength: f32,
    pub radii: Vec2,
    pub colour: Vec3,
    translation: Vec3,
    
}



#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdSphere{
    index: u32, //to the shape container for easy removal
    colour: Vec3,
    parent_idx: UVec2,
    pub radius: f32,
    transform_determinant: f32,
    inverse_transform: Mat4,
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdCube{
    index: u32,
    colour: Vec3,
    parent_idx: UVec2,
    pub size: Vec3,
    transform_determinant: f32,
    inverse_transform: Mat4,
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdEllipse{
    index: u32,
    colour: Vec3,
    parent_idx: UVec2,
    pub radii: Vec3,
    transform_determinant: f32,
    inverse_transform: Mat4,
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdTorus{
    index: u32,
    colour: Vec3,
    parent_idx: UVec2,
    pub radii: Vec2,
    transform_determinant: f32,
    inverse_transform: Mat4,
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdCylinder{
    index: u32,
    colour: Vec3,
    parent_idx: UVec2,
    pub height: f32,
    pub radius: f32,
    transform_determinant: f32,
    inverse_transform: Mat4,
}

#[derive(Component, ShaderType, Default, Debug, Clone, Copy, Reflect)]
pub struct SdCone{
    index: u32,
    colour: Vec3,
    parent_idx: UVec2,
    pub height: f32,
    pub sincos: Vec2,
    transform_determinant: f32,
    inverse_transform: Mat4,
}


//bvh functions

impl Obb {
    
    fn get_local_corners(&self) -> [Vec3; 8] {
        let hs = self.size;
        [
            Vec3::new(-hs.x, -hs.y, -hs.z),
            Vec3::new( hs.x, -hs.y, -hs.z),
            Vec3::new( hs.x,  hs.y, -hs.z),
            Vec3::new(-hs.x,  hs.y, -hs.z),
            Vec3::new(-hs.x, -hs.y,  hs.z),
            Vec3::new( hs.x, -hs.y,  hs.z),
            Vec3::new( hs.x,  hs.y,  hs.z),
            Vec3::new(-hs.x,  hs.y,  hs.z),
        ]
    }

    
    fn get_world_corners(&self) -> [Vec3; 8] {
        let local_corners = self.get_local_corners();
        let rotation = self.rotation;
        let center = self.center;
        let mut world_corners = [Vec3::ZERO; 8];

        for (i, corner) in local_corners.iter().enumerate() {
            world_corners[i] = rotation * *corner + center;
        }

        world_corners
    }

    
    fn compute_aabb(&self) -> (Vec3, Vec3) {
        let world_corners = self.get_world_corners();

        // Initialize min and max with the first corner
        let mut min = world_corners[0];
        let mut max = world_corners[0];

        // Iterate through the remaining corners to find the min and max
        for corner in &world_corners[1..] {
            min = min.min(*corner - 0.1);
            max = max.max(*corner + 0.1);
        }

        (min, max)
    }
}

fn generate_sphere_aabb(sphere: SdSphere) -> Aabb{
    let srt = sphere.inverse_transform.inverse().to_scale_rotation_translation();
    
    let obb = Obb{
        center: srt.2,
        size: sphere.radius * srt.0 * 1.5,
        rotation: srt.1,
    };

    let (smin, smax) = obb.compute_aabb();

    Aabb{min: smin, max: smax}
}

fn generate_cube_aabb(cube: SdCube) -> Aabb{
    let srt = cube.inverse_transform.inverse().to_scale_rotation_translation();
    let obb = Obb{
        center: srt.2,
        size: cube.size * srt.0 * 1.5,
        rotation: srt.1,
    };
    let (smin, smax) = obb.compute_aabb();

    Aabb{min: smin, max: smax}
}

fn generate_ellipse_aabb(ellipse: SdEllipse) -> Aabb{
    let srt = ellipse.inverse_transform.inverse().to_scale_rotation_translation();
    let obb = Obb{
        center: srt.2,
        size: ellipse.radii * srt.0 * 1.5,
        rotation: srt.1,
    };
    let (smin, smax) = obb.compute_aabb();

    Aabb{min: smin, max: smax}
}

fn generate_torus_aabb(torus: SdTorus) -> Aabb{
    let srt = torus.inverse_transform.inverse().to_scale_rotation_translation();
    let obb = Obb{
        center: srt.2,
        size: torus.radii.x + torus.radii.y * srt.0 * 1.5,
        rotation: srt.1,
    };
    let (smin, smax) = obb.compute_aabb();

    Aabb{min: smin, max: smax}
}

fn generate_cylinder_aabb(cylinder: SdCylinder) -> Aabb {
    let srt = cylinder.inverse_transform.inverse().to_scale_rotation_translation();
    let obb = Obb {
        center: srt.2,
        size: vec3(
            cylinder.radius * srt.0.x * 2.0,
            cylinder.height * srt.0.y,
            cylinder.radius * srt.0.z * 2.0,
        ) * 1.5,
        rotation: srt.1,
    };
    let (smin, smax) = obb.compute_aabb();

    Aabb { min: smin, max: smax }
}
fn generate_cone_aabb(cone: SdCone) -> Aabb {
    let srt = cone.inverse_transform.inverse().to_scale_rotation_translation();
    let base_radius = cone.height * (cone.sincos.x / cone.sincos.y);
    let obb = Obb {
        center: srt.2 + vec3(0.0, cone.height * 0.5 * srt.0.y, 0.0),
        size: vec3(
            base_radius * srt.0.x,
            cone.height * 0.5 * srt.0.y,
            base_radius * srt.0.z,
        ) * 1.5,
        rotation: srt.1,
    };
    let (smin, smax) = obb.compute_aabb();

    Aabb { min: smin, max: smax }
}

fn aabb_union(a: Aabb, b: Aabb) -> Aabb{
    Aabb { min: Vec3::min(a.min, b.min), max: Vec3::max(a.max, b.max) }
}

fn aabb_area(a: Aabb) -> f32 {
    let d = a.max - a.min;
    2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
}

fn refit_leaf(shape_idx: UVec2, 
    container: &ShapeContainer, 
    nodes: Arc<Mutex<Vec<BvhNode>>>, 
    root_index: Arc<Mutex<u32>>,
){

    let loop_count: u32 = 4;
    let root = *root_index.lock().unwrap();
    let mut nodes_1 = nodes.lock().unwrap();
    let spheres =  container.spheres.lock().unwrap();
    let cubes =  container.cubes.lock().unwrap();
    let ellipses = container.ellipses.lock().unwrap();
    let toruses = container.toruses.lock().unwrap();
    let cylinders = container.cylinders.lock().unwrap();
    let cones = container.cones.lock().unwrap();

    let leaf_index = match shape_idx.x{
        1 => spheres[shape_idx.y as usize].parent_idx.y,
        2 => cubes[shape_idx.y as usize].parent_idx.y,
        3 => ellipses[shape_idx.y as usize].parent_idx.y,
        4 => toruses[shape_idx.y as usize].parent_idx.y,
        5 => cylinders[shape_idx.y as usize].parent_idx.y,
        6 => cones[shape_idx.y as usize].parent_idx.y,
        _ => panic!("the shape idx was {}", shape_idx.y),
    };

    let aabb = match shape_idx.x{
        
        1 => generate_sphere_aabb(spheres[shape_idx.y as usize]),
        2 => generate_cube_aabb(cubes[shape_idx.y as usize]),
        3 => generate_ellipse_aabb(ellipses[shape_idx.y as usize]),
        4 => generate_torus_aabb(toruses[shape_idx.y as usize]),
        5 => generate_cylinder_aabb(cylinders[shape_idx.y as usize]),
        6 => generate_cone_aabb(cones[shape_idx.y as usize]),
        _ => panic!("the shape idx was {}", shape_idx),
    };

    nodes_1[leaf_index as usize].aabb = aabb;

    

    //refit the tree
     
    let mut refit_parent_idx = nodes_1[leaf_index as usize].o_p_idx.y;
    while refit_parent_idx != 0 {
        let child1 = nodes_1[refit_parent_idx as usize].child1;
        let child2 = nodes_1[refit_parent_idx as usize].child2;
        let aabb1 = nodes_1[child1.y as usize].aabb;
        let aabb2 = nodes_1[child2.y as usize].aabb;
        //println!("just checking");
        nodes_1[refit_parent_idx as usize].aabb = aabb_union(aabb1, aabb2);

        

        if refit_parent_idx == root{
            //refit_parent_idx = nodes_1[refit_parent_idx as usize].o_p_idx.y;
            break; //return if root or leaf
        }
    
        
        let node_parent = nodes_1[refit_parent_idx as usize].o_p_idx.y;
        let cost1 = aabb_area(nodes_1[node_parent as usize].aabb);
        
        let node_sibling = if nodes_1[node_parent as usize].child1 == uvec2(0, refit_parent_idx) {
            nodes_1[node_parent as usize].child2.y
        }else{
            nodes_1[node_parent as usize].child1.y
        };
        
    
        let limit = usize::max(nodes_1.len() - 2 - loop_count as usize, 0); 
    
        let mut i = nodes_1.len() - 2;
    
        while i > 0 || i > limit{
            if i == root as usize || nodes_1[i].child1.x != 0{
                i = i.saturating_sub(2);
                continue; //skips root and leaf nodes
            }
    
            let i_parent = nodes_1[i].o_p_idx.y;
    
            let cost2 = aabb_area(nodes_1[i_parent as usize].aabb);
            let cost3 = cost1 + cost2;
    
            let i_sibling = if nodes_1[i_parent as usize].child1 == uvec2(0, i as u32) {
                nodes_1[i_parent as usize].child2.y
            }else{
                nodes_1[i_parent as usize].child1.y
            };
    
            let cost4 = aabb_area(aabb_union(nodes_1[refit_parent_idx as usize].aabb, nodes_1[i_sibling as usize].aabb)) + aabb_area(aabb_union(nodes_1[i].aabb, nodes_1[node_sibling as usize].aabb));
    
    
            if cost4 <= cost3 {
                nodes_1.swap(i, refit_parent_idx as usize);
                nodes_1[i_parent as usize].aabb = aabb_union(nodes_1[refit_parent_idx as usize].aabb, nodes_1[i_sibling as usize].aabb);
                nodes_1[node_parent as usize].aabb = aabb_union(nodes_1[i].aabb, nodes_1[node_sibling as usize].aabb);
    
            }
    
            i = i.saturating_sub(2);
            
        }
    
       



        refit_parent_idx = nodes_1[refit_parent_idx as usize].o_p_idx.y;
    }

    drop(cubes);
    drop(spheres);
    drop(ellipses);
    drop(toruses);
    drop(cylinders);
    drop(cones);
    drop(nodes_1);
    

    //maybe do tree rotations

}


fn insert_leaf(
    shape_idx: UVec2, 
    container: &ShapeContainer, 
    nodes: Arc<Mutex<Vec<BvhNode>>>, 
    node_count: Arc<Mutex<u32>>, 
    root_index: Arc<Mutex<u32>>,
) 
{

    let mut spheres =  container.spheres.lock().unwrap();
    let mut cubes =  container.cubes.lock().unwrap();
    let mut ellipses = container.ellipses.lock().unwrap();
    let mut toruses = container.toruses.lock().unwrap();
    let mut cylinders = container.cylinders.lock().unwrap();
    let mut cones = container.cones.lock().unwrap();
    let aabb = match shape_idx.x{
        
        1 => generate_sphere_aabb(spheres[shape_idx.y as usize]),
        2 => generate_cube_aabb(cubes[shape_idx.y as usize]),
        3 => generate_ellipse_aabb(ellipses[shape_idx.y as usize]),
        4 => generate_torus_aabb(toruses[shape_idx.y as usize]),
        5 => generate_cylinder_aabb(cylinders[shape_idx.y as usize]),
        6 => generate_cone_aabb(cones[shape_idx.y as usize]),
        _ => panic!("the shape idx was {}", shape_idx),
    };

    let mut leaf = BvhNode{
        child1: shape_idx,
        aabb: aabb,
        ..Default::default()
    };

    let node_count_clone = Arc::clone(&node_count);
    let nodes_clone = Arc::clone(&nodes);
    let root_index_clone = Arc::clone(&root_index);

    let mut node_count_1 = node_count_clone.lock().unwrap();
    let mut nodes_1 = nodes_clone.lock().unwrap();
    let mut root_index_1 = root_index_clone.lock().unwrap();

    if *node_count_1 == 0 {
        *root_index_1 = 1; 
        nodes_1.push(BvhNode::default()); 
        leaf.o_p_idx = uvec2(1, 0); 
        nodes_1.push(leaf);
        
        *node_count_1 += 1;
        println!("returned!");
        return;
    }   
    drop(node_count_1);
    drop(nodes_1);
    drop(root_index_1);

    let sibling = pick_best(Arc::clone(&root_index), Arc::clone(&nodes), aabb);

    let mut node_count_1 = node_count_clone.lock().unwrap();
    let mut nodes_1 = nodes_clone.lock().unwrap();
    let mut root_index_1 = root_index_clone.lock().unwrap();

    let leaf_index = nodes_1.len() as u32; 
    leaf.o_p_idx = UVec2::new(leaf_index, 0);
    nodes_1.push(leaf);



    *node_count_1 += 1;
    let old_parent_idx = nodes_1[sibling as usize].o_p_idx.y; 
    let new_parent_idx = nodes_1.len() as u32;
    let mut new_parent = BvhNode{o_p_idx: UVec2::new(new_parent_idx, old_parent_idx), ..Default::default()}; //create the new node but don't push it just yet
    nodes_1.push(new_parent);
    *node_count_1 += 1;
    new_parent.aabb = aabb_union(aabb, nodes_1[sibling as usize].aabb); //new parent aabb is union of leaf and sibling aabb
    if old_parent_idx != 0 {
        //sibling is not the root
        if nodes_1[old_parent_idx as usize].child1.y == sibling {
            nodes_1[old_parent_idx as usize].child1 = uvec2(0, new_parent_idx);
        }
        else {
            nodes_1[old_parent_idx as usize].child2 = uvec2(0, new_parent_idx);
        }
        nodes_1[new_parent_idx as usize].child1 = uvec2(0, sibling);
        nodes_1[new_parent_idx as usize].child2 = uvec2(0, leaf_index);
        nodes_1[sibling as usize].o_p_idx.y = new_parent_idx;
        nodes_1[leaf_index as usize].o_p_idx.y = new_parent_idx;
    }
    else {
        //sibling was the root
        nodes_1[new_parent_idx as usize].child1 = uvec2(0, sibling);
        nodes_1[new_parent_idx as usize].child2 = uvec2(0, leaf_index);
        nodes_1[sibling as usize].o_p_idx.y = new_parent_idx;
        nodes_1[leaf_index as usize].o_p_idx.y = new_parent_idx;
        *root_index_1 = new_parent_idx;

    }


    //set the leaf as the parent index of 

    match shape_idx.x{
        1 => spheres[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        2 => cubes[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        3 => ellipses[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        4 => toruses[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        5 => cylinders[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        6 => cones[shape_idx.y as usize].parent_idx = uvec2(0, leaf_index),
        _ => panic!("the shape idx was {}", shape_idx),
        
    }

    drop(node_count_1);
    drop(nodes_1);
    drop(root_index_1);

    
    
    

    //refit the tree

    let loop_count: u32 = 4;
    let root = *root_index.lock().unwrap();
    let mut nodes_1 = nodes.lock().unwrap();
     
    let mut refit_parent_idx = nodes_1[leaf_index as usize].o_p_idx.y;
    while refit_parent_idx != 0 {
        let child1 = nodes_1[refit_parent_idx as usize].child1;
        let child2 = nodes_1[refit_parent_idx as usize].child2;
        let aabb1 = nodes_1[child1.y as usize].aabb;
        let aabb2 = nodes_1[child2.y as usize].aabb;
        //println!("just checking");
        nodes_1[refit_parent_idx as usize].aabb = aabb_union(aabb1, aabb2);

        if refit_parent_idx == root || refit_parent_idx == 0{
            //refit_parent_idx = nodes_1[refit_parent_idx as usize].o_p_idx.y;
            break; //return if root or leaf
        }

        let node_parent = nodes_1[refit_parent_idx as usize].o_p_idx.y;
        let cost1 = aabb_area(nodes_1[node_parent as usize].aabb);
        
        let node_sibling = if nodes_1[node_parent as usize].child1 == uvec2(0, refit_parent_idx) {
            nodes_1[node_parent as usize].child2.y
        }else{
            nodes_1[node_parent as usize].child1.y
        };
        
    
        let limit = usize::max(nodes_1.len() - 2 - loop_count as usize, 0); 
    
        let mut i = nodes_1.len() - 2;
    
        while i > 0 || i > limit{
            if i == root as usize || nodes_1[i].child1.x != 0{
                i = i.saturating_sub(2);
                continue; //skips root and leaf nodes
            }
            let i_parent = nodes_1[i].o_p_idx.y;
            let cost2 = aabb_area(nodes_1[i_parent as usize].aabb);
            let cost3 = cost1 + cost2;
            let i_sibling = if nodes_1[i_parent as usize].child1 == uvec2(0, i as u32) {
                nodes_1[i_parent as usize].child2.y
            }else{
                nodes_1[i_parent as usize].child1.y
            };
            let cost4 = aabb_area(aabb_union(nodes_1[refit_parent_idx as usize].aabb, nodes_1[i_sibling as usize].aabb)) + aabb_area(aabb_union(nodes_1[i].aabb, nodes_1[node_sibling as usize].aabb));
            if cost4 <= cost3 {
                nodes_1.swap(i, refit_parent_idx as usize);
                nodes_1[i_parent as usize].aabb = aabb_union(nodes_1[refit_parent_idx as usize].aabb, nodes_1[i_sibling as usize].aabb);
                nodes_1[node_parent as usize].aabb = aabb_union(nodes_1[i].aabb, nodes_1[node_sibling as usize].aabb);
    
            }
            i = i.saturating_sub(2);
        }

       
        refit_parent_idx = nodes_1[refit_parent_idx as usize].o_p_idx.y;
    }
    drop(nodes_1);
    drop(cubes);
    drop(spheres);
    drop(ellipses);
    drop(toruses);
    drop(cylinders);
    drop(cones);
    


    
}


fn pick_best(
    root_index: Arc<Mutex<u32>>,
    nodes: Arc<Mutex<Vec<BvhNode>>>,
    new_aabb: Aabb,
) -> u32 {

    let mut current_idx = *root_index.lock().unwrap();

    let mut current_cost = aabb_area(aabb_union(nodes.lock().unwrap()[current_idx as usize].aabb, new_aabb));

    let default_idx = UVec2::new(0,0);
    
    //start at root node, get the SAH

    loop {

    //if both children are more nodes (this means internal node) (neither of the internal nodes can be the root / default UVec2 either, so we dont end up going in some loop)
    if nodes.lock().unwrap()[current_idx as usize].child1.x == 0 && 
    nodes.lock().unwrap()[current_idx as usize].child2.x == 0 && 
    nodes.lock().unwrap()[current_idx as usize].child1 != default_idx && 
    nodes.lock().unwrap()[current_idx as usize].child2 != default_idx{
        
        let child1 = nodes.lock().unwrap()[current_idx as usize].child1.y;
        let child2 = nodes.lock().unwrap()[current_idx as usize].child2.y;

        let child1_cost = aabb_area(aabb_union(nodes.lock().unwrap()[child1 as usize].aabb, new_aabb));
        let child2_cost = aabb_area(aabb_union(nodes.lock().unwrap()[child2 as usize].aabb, new_aabb));
        //println!("hello 2! (internal node)");
        if child1_cost <= child2_cost{ // if child 1 is better or equal
            current_idx = child1; 
            current_cost = child1_cost;
            continue;
        }
        else{ //if child 2 is better
            current_idx = child2; 
            current_cost = child2_cost;
            continue;
        }

    }
    else{ //if at least one of the children is not a node or is default (this means leaf node, usually) 
        let child1 = nodes.lock().unwrap()[current_idx as usize].child1;
        let child2 = nodes.lock().unwrap()[current_idx as usize].child2;
        if (child1.x != 0 && child2.x != 0) || (child1 == default_idx && child2 == default_idx){ //if both children are not nodes or both default / root (this probably should not be possible but just incase)
            break; //just go with what it is currently
        }
        else if child1.x != 0 || child1 == default_idx{ // first child of the current is not a node or is [0, 0] (the default, and also root)
            let child2_cost = aabb_area(aabb_union(nodes.lock().unwrap()[child2.y as usize].aabb, new_aabb));

            //println!("hello 4! (leaf node)");
            if child2_cost >= current_cost { // if the child cost is not better
                //break out of the whole loop
                break;
            }
            else{ //if the child cost was better
                if child2 == default_idx {
                    break;
                }
                current_idx = child2.y; //make it the new current and check for more
                current_cost = child2_cost;
                continue;
            }
        }
        
        else if child2.x != 0 || child2 == default_idx{ //second child of the current is not a node or is default
            let child1_cost = aabb_area(aabb_union(nodes.lock().unwrap()[child1.y as usize].aabb, new_aabb));

            //println!("hello 5! (leaf node)");
            if child1_cost >= current_cost { // if the child cost is not better
                //break out of the whole loop
                break;
            }
            else{ //if the child cost was better
                if child1 == default_idx {
                    break;
                }
                current_idx = child1.y; //make it the new current and check for more
                current_cost = child1_cost;
                continue;
            }

        }
        else{//just break incase of any other monkey business
            break;
        }
        

    }

    
    }

    
    current_idx //return the idx of the node with the current (or best found) cost
}



fn prepare_node_removal(
    node1: u32, //the node that is being removed
    nodes_lock: Arc<Mutex<Vec<BvhNode>>>, //the vector of nodes
    node_count: Arc<Mutex<u32>>, 
    root_index: Arc<Mutex<u32>>,
    container: &ShapeContainer,
) 
{

    //first, grab node 1's parent and find which child is equal to node1, setting the child to [0, 0] (nothing there)
    //alternatively, if node1 is the root node, then don't bother fixing children, but make whatever child of it that isn't [0, 0] but is a node, the new root.
    //grab the last node of the vec, checking that node1 isn't the last node of the vec, if it isn't, save the index of the last vec of the node as node2.
    //then, call the swap_remove function on node 1. if node1 was the last element, return and remove 1 from node_count.
    //grab node2 and fix it's parent and children.

    let mut nodes = nodes_lock.lock().unwrap();

    let default = uvec2(0, 0);
    let child1 = nodes[node1 as usize].child1;
    let child2 = nodes[node1 as usize].child2;
    let parent1 = nodes[node1 as usize].o_p_idx.y;

    if parent1 == 0 {   // node1 is root node

        if (child2.x != 0 || child2 == default) && child1 != default{ //if child2 is either not a node or unused AND child1 isn't unused then.. 
            *root_index.lock().unwrap() = child1.y; //set root index to child1

        }else if (child1.x != 0 || child1 == default) && child2 != default{ //if child1 is either not a node or unused AND child2 isn't unused then.. 
            *root_index.lock().unwrap() = child2.y; //set root index to child2

        }
    }
    let last_idx = (nodes.len() - 1) as u32;
    nodes.swap_remove(node1 as usize);
    *node_count.lock().unwrap() -= 1;
    if node1 != last_idx{ //if node2 is not eq
        nodes[node1 as usize].o_p_idx.x = node1;
        let oldparentlast = nodes[node1 as usize].o_p_idx.y;
        if nodes[oldparentlast as usize].child1 == uvec2(0, last_idx) {
            nodes[oldparentlast as usize].child1.y = node1;
        }
        else{
            nodes[oldparentlast as usize].child2.y = node1;
        }
        let oldchild1 = nodes[node1 as usize].child1;
        let oldchild2 = nodes[node1 as usize].child2;

        let mut spheres =  container.spheres.lock().unwrap();
        let mut cubes =  container.cubes.lock().unwrap();
        let mut ellipses = container.ellipses.lock().unwrap();
        let mut toruses = container.toruses.lock().unwrap();
        let mut cylinders = container.cylinders.lock().unwrap();
        let mut cones = container.cones.lock().unwrap();
        if oldchild1 != default { //handle swapped child 1
            match oldchild1.x {
                0 => nodes[oldchild1.y as usize].o_p_idx.y = node1,
                1 => spheres[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                2 => cubes[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                3 => ellipses[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                4 => toruses[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                5 => cylinders[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                6 => cones[oldchild1.y as usize].parent_idx = uvec2(0, node1),
                _ => panic!("missing case for node fixing, check the prepare_node_removal function")
            }
        }
        if oldchild2 != default { //handle swapped child 2
            match oldchild2.x {
                0 => nodes[oldchild2.y as usize].o_p_idx.y = node1,
                1 => spheres[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                2 => cubes[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                3 => ellipses[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                4 => toruses[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                5 => cylinders[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                6 => cones[oldchild2.y as usize].parent_idx = uvec2(0, node1),
                _ => panic!("missing case for node fixing, check the prepare_node_removal function")
            }
        }
    }
}






fn remove_leaf(
    shape_idx: UVec2, 
    container: &ShapeContainer, 
    nodes: Arc<Mutex<Vec<BvhNode>>>, 
    node_count: Arc<Mutex<u32>>, 
    root_index: Arc<Mutex<u32>>,
) {

    //lock mutexes

    let spheres =  container.spheres.lock().unwrap();
    let cubes =  container.cubes.lock().unwrap();
    let ellipses = container.ellipses.lock().unwrap();
    let toruses = container.toruses.lock().unwrap();
    let cylinders = container.cylinders.lock().unwrap();
    let cones = container.cones.lock().unwrap();

    //validate shape type


    let leaf_idx = match shape_idx.x {
        1 => spheres[shape_idx.y as usize].parent_idx,
        2 => cubes[shape_idx.y as usize].parent_idx,
        3 => ellipses[shape_idx.y as usize].parent_idx,
        4 => toruses[shape_idx.y as usize].parent_idx,
        5 => cylinders[shape_idx.y as usize].parent_idx,
        6 => cones[shape_idx.y as usize].parent_idx,
        _ => panic!("the shape idx was {}", shape_idx),
    };

    if leaf_idx.x != 0 {
        return; //figure this out (when the parent is not a node)
    }

    drop(spheres);
    drop(cubes);
    drop(ellipses);
    drop(toruses);
    drop(cylinders);
    drop(cones);

    let first_was_root = leaf_idx.y == *root_index.lock().unwrap(); 

    let parent_idx = nodes.lock().unwrap()[leaf_idx.y as usize].o_p_idx.y;

    prepare_node_removal(leaf_idx.y, Arc::clone(&nodes), Arc::clone(&node_count), Arc::clone(&root_index), container);

    refit_leaf(uvec2(0,parent_idx), container, Arc::clone(&nodes), Arc::clone(&root_index));

    if !first_was_root{

        prepare_node_removal(parent_idx, Arc::clone(&nodes), Arc::clone(&node_count), Arc::clone(&root_index), container);

    }



}

/* 
fn rotate_tree(
    node_idx: u32, 
    nodes: Arc<Mutex<Vec<BvhNode>>>, 
    root_index: Arc<Mutex<u32>>,
    loop_count: u32,
) {

    //if node idx is root then return

    //get cost of the parent of the node index and save it as cost 1

    //counter of i = -2 of index of node vec, -2 or 3 each time, stop when either i is less than or 1 or when i is less than some threshold like 10 or something

    //skip i if it's the root node or default node

    //get cost of parent of i and save it as cost 2

    //add cost 1 and 2 together to make 3

    //calculate cost of 3 where node and i are swapped, save as cost 4

    //if cost 4 is lower then or equal to cost 3, swap the nodes

    //end of loop

    let root = *root_index.lock().unwrap();

    let mut nodeslock = nodes.lock().unwrap();

    if node_idx == root || node_idx == 0{
        return; //return if root or default
    }

    
    let node_parent = nodeslock[node_idx as usize].o_p_idx.y;
    let cost1 = aabb_area(nodeslock[node_parent as usize].aabb);
    
    let node_sibling = if nodeslock[node_parent as usize].child1 == uvec2(0, node_idx) {
        nodeslock[node_parent as usize].child2.y
    }else{
        nodeslock[node_parent as usize].child1.y
    };
    

    let limit = usize::max(nodeslock.len() - 2 - loop_count as usize, 0); 

    let mut i = nodeslock.len() - 2;

    while i > 0 || i > limit{
        if i == root as usize || nodeslock[i].child1.x != 0{
            i = i.saturating_sub(2);
            continue; //skips root and leaf nodes
        }

        let i_parent = nodeslock[i].o_p_idx.y;

        let cost2 = aabb_area(nodeslock[i_parent as usize].aabb);
        let cost3 = cost1 + cost2;

        let i_sibling = if nodeslock[i_parent as usize].child1 == uvec2(0, i as u32) {
            nodeslock[i_parent as usize].child2.y
        }else{
            nodeslock[i_parent as usize].child1.y
        };

        let cost4 = aabb_area(aabb_union(nodeslock[node_idx as usize].aabb, nodeslock[i_sibling as usize].aabb)) + aabb_area(aabb_union(nodeslock[i].aabb, nodeslock[node_sibling as usize].aabb));


        if cost4 <= cost3 {
            nodeslock.swap(i, node_idx as usize);
            nodeslock[i_parent as usize].aabb = aabb_union(nodeslock[node_idx as usize].aabb, nodeslock[i_sibling as usize].aabb);
            nodeslock[node_parent as usize].aabb = aabb_union(nodeslock[i].aabb, nodeslock[node_sibling as usize].aabb);

        }

        i = i.saturating_sub(2);
        
    }

}
    */