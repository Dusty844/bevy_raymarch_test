#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_render::view::View

struct Aabb{
    min: vec3<f32>,
    max: vec3<f32>,
}


struct BvhNode{
    o_p_idx: vec2<u32>,
    aabb: Aabb,
    child1: vec2<u32>,
    child2: vec2<u32>,
}


struct SdSphere{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    radius: f32,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdCube{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    size: vec3<f32>,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdEllipse{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    radii: vec3<f32>,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdTorus{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    radii: vec2<f32>,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdCylinder{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    height: f32,
    radius: f32,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdCone{
    index: u32,
    colour: vec3<f32>,
    parent_index: vec2<u32>,
    height: f32,
    sincos: vec2<f32>,
    transform_determinant: f32,
    inverse_transform: mat4x4<f32>,
}

struct SdDirectionalLight{
    strength: f32,
    colour: vec3<f32>,
    direction: vec3<f32>,
}

struct SdPositionalLight{
    strength: f32,
    radii: vec2<f32>,
    colour: vec3<f32>,
    translation: vec3<f32>,
}

struct Intersection{
    t: f32,
    colour: vec3<f32>,
    k: f32,
    normal: vec3<f32>,
    col_mod: vec3<f32>,
    hit_pos: vec3<f32>,
    min_dist: f32,
    hit: bool,
    
}

struct ShadowIntersection{
    min_dist: f32,
    hit: bool,
    t: f32,
}

struct RaymarchSettings{
    lower_colour: vec3<f32>,
    middle_colour: vec3<f32>,
    upper_colour: vec3<f32>,
    max_distance: f32,
    powers: vec2<f32>,
    shadow_power: f32,

}


@group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(1) var<uniform> position: vec3<f32>;
@group(2) @binding(2) var<uniform> forward: vec3<f32>;
@group(2) @binding(3) var<uniform> horizontal: vec3<f32>;
@group(2) @binding(4) var<uniform> vertical: vec3<f32>;
@group(2) @binding(5) var<uniform> fov: f32;
@group(2) @binding(6) var<uniform> root_index: u32;
@group(2) @binding(7) var<uniform> raymarch_settings: RaymarchSettings;
@group(2) @binding(8) var<storage, read> nodes: array<BvhNode>;
@group(2) @binding(9) var<storage, read> dir_lights: array<SdDirectionalLight>;
@group(2) @binding(10) var<storage, read> pos_lights: array<SdPositionalLight>;
@group(2) @binding(11) var<storage, read> spheres: array<SdSphere>;
@group(2) @binding(12) var<storage, read> cubes: array<SdCube>;
@group(2) @binding(13) var<storage, read> ellipses: array<SdEllipse>;
@group(2) @binding(14) var<storage, read> toruses: array<SdTorus>;
@group(2) @binding(15) var<storage, read> cylinders: array<SdCylinder>;
@group(2) @binding(16) var<storage, read> cones: array<SdCone>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var max_distance: f32 = 3000.0;
    var fov_rad = radians(fov);
    var scale_factor = tan(fov_rad / 2.0);

    var uv = (in.uv.xy * 2.0) - 1.0;
    let resolution = view.viewport.zw;
    uv.x *= resolution.x / resolution.y * scale_factor;
    uv.y *= scale_factor;
    var camera_origin = position;
    var temp_origin = camera_origin + forward + (uv.x * horizontal) + (-uv.y * vertical);
    var ray_direction = normalize(temp_origin - camera_origin);
    var inv_ray_direction = 1.0 / ray_direction;

    var inter = initial_intersect(camera_origin, ray_direction, inv_ray_direction);

    var col = vec3f();

    if inter.hit {
        //var n = inter.normal * vec3f(0.5) + vec3(0.5);

        var pos = inter.hit_pos;

        col = inter.colour;

        var shadow = 1.0;

        var shadow_id = 1 / dir_lights[0].direction;
        
        shadow = shadow_intersect(inter.hit_pos + inter.normal * 0.2, dir_lights[0].direction, shadow_id);

        var nol = max(dot(inter.normal, dir_lights[0].direction) + 0.1, 0.0) * shadow;


        var ambient = vec3f(0.02, 0.021, 0.02);

        col = col * (nol + ambient);
        col = pow(col, vec3f(0.4545));
        //let dist = inter.t / raymarch_settings.max_distance;
        //col = vec3f(dist, dist, dist);
        return vec4f(col, 1.0);
    }
    else{



        var testray: f32 = (ray_direction.y + 1);

        var lower = raymarch_settings.lower_colour;
        var middle = raymarch_settings.middle_colour;
        var upper = raymarch_settings.upper_colour;

        var mix1 = mix(lower, middle, pow(clamp((testray), 0.0, 1.0), raymarch_settings.powers.x));
        var mix2 = mix(mix1, upper, pow(clamp(testray - 1.0, 0.0, 1.0), raymarch_settings.powers.y));

        
        



        var sun = clamp(pow(dot(dir_lights[0].direction, ray_direction), 500.0) * 12.0, 0.0, 1.0);



        col = mix(mix2, dir_lights[0].colour, sun);

        col = pow(col, vec3f(0.4545));
        //let dist = inter.t / raymarch_settings.max_distance;
        //col = vec3f(dist, dist, dist);
        return vec4f(col, 1.0);
        //return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    

    

    
    

}

fn initial_intersect(
    ray_o: vec3<f32>, 
    ray_d: vec3<f32>,
    ray_id: vec3<f32>,
) -> Intersection{
    let max_distance = raymarch_settings.max_distance;
    var inter: Intersection;
    inter.hit = false;
    inter.t = max_distance;
    inter.colour = vec3<f32>(0.0, 0.0, 0.0);
    inter.k = 0.0;
    inter.normal = vec3<f32>(0.0, 0.0, 0.0);
    inter.col_mod = vec3<f32>(1.0, 1.0, 1.0);
    inter.hit_pos = ray_o;

    let default_idx = vec2<u32>(0, 0);
    
    var stack: array<u32, 20>;
    var stackPtr: i32 = 0;

    // Push the root index onto the stack
    stack[stackPtr] = root_index;
    stackPtr = stackPtr + 1;

    var hit = false;

    loop {
         if (stackPtr == 0) {
            break;
        }

        stackPtr = stackPtr - 1;
        let index: u32 = stack[stackPtr];

        let currentNode: BvhNode = nodes[index];

        let current_dists = intersect_aabb_dist(ray_o, ray_id, currentNode.aabb, inter.t);

        if (current_dists.y <= 0.0 || current_dists.x > inter.t){
            continue;
        }

        var child1 = currentNode.child1;
        var child2 = currentNode.child2;

        if (child1.x != 0) {
            inter = raymarch(child1, ray_o, ray_d, current_dists, inter);

        }else{

            var dists1 = intersect_aabb_dist(ray_o, ray_id, nodes[child1.y].aabb, inter.t);
            var dists2 = intersect_aabb_dist(ray_o, ray_id, nodes[child2.y].aabb, inter.t);

            if (dists1.x < dists2.x) {
                if (dists2.y > 0.0 && dists2.x < inter.t){
                    stack[stackPtr] = currentNode.child2.y;
                    stackPtr = stackPtr + 1;
                }
                if (dists1.y > 0.0 && dists1.x < inter.t){
                    stack[stackPtr] = currentNode.child1.y;
                    stackPtr = stackPtr + 1;
                }
            } else {
                if (dists1.y > 0.0 && dists1.x < inter.t){
                    stack[stackPtr] = currentNode.child1.y;
                    stackPtr = stackPtr + 1;
                }
                if (dists2.y > 0.0 && dists2.x < inter.t){
                    stack[stackPtr] = currentNode.child2.y;
                    stackPtr = stackPtr + 1;
                }
            }


        }

    }

    return inter;
}


fn intersect_aabb_dist(
    ray_origin: vec3<f32>,
    inv_ray_direction: vec3<f32>,
    aabb: Aabb,
    max_distance: f32,
) -> vec2<f32> {
    let t0 = (aabb.min - ray_origin) * inv_ray_direction;
    let t1 = (aabb.max - ray_origin) * inv_ray_direction;
    
    let tmin = min(t0, t1);
    let tmax = max(t0, t1);

    let dst_a = max(max(tmin.x, tmin.y), tmin.z);
    let dst_b = min(min(tmax.x, tmax.y), tmax.z);

    let dst_to_box = max(0.0, dst_a);
    let adjusted_dst_b = min(dst_b, max_distance);

    // Calculate distance inside the box considering max_distance
    let dst_inside_box = max(0.0, adjusted_dst_b - dst_to_box);
    
    return vec2<f32>(dst_to_box, dst_inside_box);
}

fn raymarch(
    shape_idx: vec2<u32>,
    ray_o: vec3<f32>, 
    ray_d: vec3<f32>,
    dists: vec2<f32>,
    intersect: Intersection,
) -> Intersection {
    
    var inter = intersect;
    var count: u32 = 0;
    var t = dists.x;
    
    var max = dists.x + dists.y;
    //let step_size: f32 = dists.y / 48.0; // Define a suitable step size

    loop{
        if (count >= 64 || t >= max){
            break;
        }
        var dist = map(ray_o + ray_d * t, shape_idx);
        if (dist < (0.001 * t * 2)) {
            if (t < intersect.t) {
            var pos = ray_o + ray_d * t;
            var normal = calc_normal(pos, dist, vec3f(0.0, 0.0, 0.0), 1.0, shape_idx);
            

            inter.normal = normal;
            
            inter.colour = get_colour(shape_idx);
            
            inter.hit_pos = ray_o + ray_d * t;

            //inter.normal = normal;
            inter.hit = true;
            inter.col_mod = vec3f(1.0, 1.0, 1.0); // Assign appropriate color based on material
            inter.t = t;
             
            }
            break; // Exit the loop after a hit
        }
        t = t + dist; // Increment t
        count = count + 1;      // Increment count
    }

    return inter;
}

fn calc_normal(p: vec3<f32>, c: f32, sp: vec3<f32>, ss: f32, idx: vec2<u32>) -> vec3<f32>{
    // Define the epsilon vector
    let eps_zero: vec2<f32> = vec2<f32>(0.001, 0.0);

    // Manually create the swizzled positions
    let pos_xyy: vec3<f32> = p + vec3<f32>(eps_zero.x, eps_zero.y, eps_zero.y);
    let pos_yxy: vec3<f32> = p + vec3<f32>(eps_zero.y, eps_zero.x, eps_zero.y);
    let pos_yyx: vec3<f32> = p + vec3<f32>(eps_zero.y, eps_zero.y, eps_zero.x);

    // Sample the SDF at the swizzled positions
    let sdf_xyy: f32 = map(pos_xyy, idx);
    let sdf_yxy: f32 = map(pos_yxy, idx);
    let sdf_yyx: f32 = map(pos_yyx, idx);

    // Construct the gradient vector by subtracting 'c' and normalizing
    let gradient: vec3<f32> = vec3<f32>(sdf_xyy, sdf_yxy, sdf_yyx) - c;
    return normalize(gradient);
} 

fn get_colour(idx: vec2<u32>) -> vec3<f32> {
    var colour: vec3<f32>;
    switch idx.x {
            default {
                colour = vec3f(0.0);
            }
            case 1u {
            let sphere = spheres[idx.y];
            colour = sphere.colour;
            }
            case 2u {
            let cube = cubes[idx.y];
            colour = cube.colour;
            }
            case 3u {
            let ellipse = ellipses[idx.y];
            colour = ellipse.colour;    
            }
            case 4u {
            let torus = toruses[idx.y];
            colour = torus.colour;    
            }
            case 5u {
            let cylinder = cylinders[idx.y];
            colour = cylinder.colour;
            }
            case 6u {
            let cone = cones[idx.y];
            colour = cone.colour;
            }
    }


    return colour;
}


fn map(p: vec3<f32>, idx: vec2<u32>) -> f32{
    var dist: f32 = 10000.0;
    switch idx.x {
        default {

        }
        case 1u {
            let sphere = spheres[idx.y];
            
            dist = SdfSphere(opTransform(p, sphere.inverse_transform), sphere.radius) * sphere.transform_determinant;
        }
        case 2u {
            let cube = cubes[idx.y];
            
            dist = SdfCube(opTransform(p, cube.inverse_transform), cube.size) * cube.transform_determinant;

        }
        case 3u {
            let ellipse = ellipses[idx.y];
            dist = SdfEllipsoid(opTransform(p, ellipse.inverse_transform), ellipse.radii) * ellipse.transform_determinant;
        }
        case 4u {
            let torus = toruses[idx.y];
            dist = SdfTorus(opTransform(p, torus.inverse_transform), torus.radii.x, torus.radii.y) * torus.transform_determinant;
        }
        case 5u {
            let cylinder = cylinders[idx.y];
            dist = SdfCylinder(opTransform(p, cylinder.inverse_transform), cylinder.height, cylinder.radius) * cylinder.transform_determinant;
        }
        case 6u {
            let cone = cones[idx.y];
            dist = SdfCone(opTransform(p, cone.inverse_transform), cone.height, cone.sincos) * cone.transform_determinant;
        }

    }
    return dist;

}

fn opTransform(p: vec3<f32>, inv_transform: mat4x4<f32>) -> vec3<f32> {
    let q = inv_transform * vec4<f32>(p, 1.0);
    return q.xyz; // Return the transformed position
}

fn SdfSphere(p: vec3f, r: f32) -> f32 {
  return length(p) - r;
}

fn SdfCube(p: vec3f, b: vec3f) -> f32 {
  let q = abs(p) - b;
  return length(max(q, vec3f(0.))) + min(max(q.x, max(q.y, q.z)), 0.);
}

fn SdfEllipsoid(p: vec3f, r: vec3f) -> f32 {
  let k0 = length(p / r);
  let k1 = length(p / (r * r));
  return k0 * (k0 - 1.) / k1;
}
fn SdfTorus(p: vec3f, R: f32, r: f32) -> f32 {
  let q = vec2f(length(p.xz) - R, p.y);
  return length(q) - r;
}

fn SdfCylinder(p: vec3f, h: f32, r: f32) -> f32 {
  let d = abs(vec2f(length(p.xz), p.y)) - vec2f(r, h);
  return min(max(d.x, d.y), 0.) + length(max(d, vec2f(0.)));
}

fn SdfCone(p: vec3f, h: f32, sincos: vec2f) -> f32 {
  return max(dot(sincos.yx, vec2f(length(p.xz), p.y)), -h - p.y);
}

fn shadow_intersect(
    ray_o: vec3<f32>, 
    ray_d: vec3<f32>,
    ray_id: vec3<f32>,
) -> f32 {
    let max_distance = raymarch_settings.max_distance; // e.g., 650.0
    let default_idx = vec2<u32>(0, 0);
    var stack: array<u32, 20>;
    var stackPtr: i32 = 0;
    var shadow_res: f32 = 1.0; // Initialize shadow factor

    // Push the root index onto the stack
    stack[stackPtr] = root_index;
    stackPtr = stackPtr + 1;

    loop {
        if (stackPtr == 0) {
            break;
        }

        stackPtr = stackPtr - 1;
        let index: u32 = stack[stackPtr];
        let currentNode: BvhNode = nodes[index];

        let current_dists = intersect_aabb_dist(ray_o, ray_id, currentNode.aabb, max_distance);

        // If there's no intersection with the current AABB, skip it
        if (current_dists.y <= 0.0) {
            continue;
        }

        let child1 = currentNode.child1;
        let child2 = currentNode.child2;

        // Check if the node is a leaf node
        if (child1.x != 0) {
            // Leaf node: Perform soft shadow raymarching
            let shape_idx = child1; // Assuming child1 encodes shape index
            let s = soft_shadow_raymarch(shape_idx, ray_o, ray_d, current_dists, raymarch_settings.shadow_power);
            shadow_res = min(shadow_res, s); // Accumulate shadow factor

            // Early termination if shadow is fully blocked
            if (shadow_res < 0.0001) {
                break;
            }
        } else {
            // Internal node: Push child nodes onto the stack in order
            let dists1 = intersect_aabb_dist(ray_o, ray_id, nodes[child1.y].aabb, max_distance);
            let dists2 = intersect_aabb_dist(ray_o, ray_id, nodes[child2.y].aabb, max_distance);

            // Order traversal based on entry distances for potential performance gains
            if (dists1.x < dists2.x) {
                if (dists2.y > 0.0) {
                    stack[stackPtr] = currentNode.child2.y;
                    stackPtr = stackPtr + 1;
                }
                if (dists1.y > 0.0) {
                    stack[stackPtr] = currentNode.child1.y;
                    stackPtr = stackPtr + 1;
                }
            } else {
                if (dists1.y > 0.0) {
                    stack[stackPtr] = currentNode.child1.y;
                    stackPtr = stackPtr + 1;
                }
                if (dists2.y > 0.0) {
                    stack[stackPtr] = currentNode.child2.y;
                    stackPtr = stackPtr + 1;
                }
            }
        }
    }

    // Clamp and smooth the final shadow factor
    shadow_res = clamp(shadow_res, 0.0, 1.0);
    shadow_res = shadow_res * shadow_res * (3.0 - 2.0 * shadow_res); // Smoothstep-like function

    return shadow_res;
}

fn shadow_raymarch(
    shape_idx: vec2<u32>,
    ray_o: vec3<f32>, 
    ray_d: vec3<f32>,
    dists: vec2<f32>,
    intersect: Intersection,
) -> Intersection {
    
    var inter = intersect;
    var count: u32 = 0;
    var t = dists.x;
    
    var max = dists.x + dists.y;
    let step_size: f32 = dists.y / 10.0; // Define a suitable step size

    loop{
        if (count >= 4 || t >= max){
            break;
        }
        var dist = map(ray_o + ray_d * t, shape_idx);
        if (dist <= (0.01)) {
            if (t < intersect.t) {
            inter.min_dist = min(inter.min_dist, dist);
            inter.hit = true;
            inter.t = t;
             
            }
            break; // Exit the loop after a hit
        }
        t = t + dist; // Increment t
        count = count + 1;      // Increment count
    }

    return inter;
}

fn soft_shadow_raymarch(
    shape_idx: vec2<u32>,      // Identifier for the shape to consider
    ray_o: vec3<f32>,          // Ray origin
    ray_d: vec3<f32>,          // Ray direction (normalized)
    dists: vec2<f32>,          // dists.x = mint, dists.y = maxt
    w: f32                     // Width parameter for shadow softness
) -> f32 {
    var res: f32 = 1.0;
    var t: f32 = dists.x;
    let maxt: f32 = dists.x + dists.y;
    let max_steps: u32 = 8u;      // Increased steps to allow deeper penetration
    let min_step: f32 = dists.y / 8.0;      // Minimum step size
    let max_step: f32 = dists.y / 2.0;       // Maximum step size

    for (var i: u32 = 0u; i < max_steps && t < maxt; i = i + 1u) {
        let pos: vec3<f32> = ray_o + ray_d * t;
        let h: f32 = map(pos, shape_idx);  // Signed distance to the nearest surface

        // Update shadow attenuation
        res = min(res, h / (w * t));

        // Increment t by clamped h to control step size
        t = t + clamp(h, min_step, max_step);

        // Terminate if shadow is fully blocked
        if (res < -1.0) {
            break;
        }
    }

    // Clamp res to a minimum of -1.0 to match the original function
    res = max(res, -1.0);

    // Apply a smoothstep-like function for smooth shadow transitions
    return 0.25 * (1.0 + res) * (1.0 + res) * (2.0 - res);
}