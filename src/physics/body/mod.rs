use crate::{
    math::vec2::Vec2, 
    physics::collider::Shape
};

pub struct Body {
    
    pub mass: f32,
    pub inv_mass: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub force: Vec2,
    pub rotation: f32,  // Rotation angle in radians
    
    // Angular properties
    pub angular_velocity: f32,  // Rotational velocity in rad/s
    pub torque: f32,  // Accumulated torque
    pub moment_of_inertia: f32,  // Rotational mass
    pub inv_moment_of_inertia: f32,  // 1/I for performance

    pub shape: Shape,
    pub gravity_enabled: bool,
    pub restitution: f32,
}

impl Body {

    const GRAVITY: Vec2 = Vec2 { x: 0.0, y: 9.81 };

    pub fn new(mass: f32, position: Vec2, shape: Shape) -> Self {
        let inv_mass = if mass != 0.0 { 1.0 / mass } else { 0.0 };
        
        let moment_of_inertia = if mass != 0.0 {
            match shape {

                Shape::Circle { radius } => 0.5 * mass * radius * radius,

                Shape::Box { width, height } => (1.0 / 12.0) * mass * (width * width + height * height),
            }
        } else {
            0.0
        };
        
        let inv_moment_of_inertia = if moment_of_inertia != 0.0 { 
            1.0 / moment_of_inertia 
        } else { 
            0.0 
        };
        
        Self {
            mass,
            inv_mass,
            velocity: Vec2::new(0.0, 0.0),
            position,
            force: Vec2::new(0.0, 0.0),
            rotation: 0.0,  // Start with no rotation
            angular_velocity: 0.0,
            torque: 0.0,
            moment_of_inertia,
            inv_moment_of_inertia,
            shape,
            gravity_enabled: false,
            restitution: 0.5, // Default medium bounce
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.force = self.force + force;
    }

    pub fn set_gravity_enabled(&mut self, enabled: bool) {
        self.gravity_enabled = enabled;
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution.clamp(0.0, 1.0);
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if self.inv_mass == 0.0 {
            return;
        }
        self.velocity = self.velocity + impulse * self.inv_mass;
    }

    pub fn apply_impulse_at_point(&mut self, impulse: Vec2, contact_point: Vec2) {
        if self.inv_mass == 0.0 {
            return;
        }
        
        // Apply linear impulse
        self.velocity = self.velocity + impulse * self.inv_mass;
        
        // Clamp velocity immediately to prevent tunneling
        const MAX_VELOCITY: f32 = 800.0;
        let speed_sq = self.velocity.length_squared();
        if speed_sq > MAX_VELOCITY * MAX_VELOCITY {
            self.velocity = self.velocity.normalize() * MAX_VELOCITY;
        }
        
        // Apply angular impulse
        if self.inv_moment_of_inertia != 0.0 {
  
            let r = contact_point - self.position;
            

            let torque_impulse = r.cross(impulse);
            
            // Apply angular impulse
            self.angular_velocity += torque_impulse * self.inv_moment_of_inertia;
            
            // Clamp angular velocity
            const MAX_ANGULAR_VELOCITY: f32 = 30.0;
            self.angular_velocity = self.angular_velocity.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
        }
    }

    pub fn apply_torque(&mut self, torque: f32) {
        self.torque += torque;
    }

    pub fn integrate(&mut self, dt: f32) {
        if self.inv_mass == 0.0 {
            return;
        }

        // Linear motion integration
        if self.gravity_enabled {
            self.apply_force(Self::GRAVITY * self.mass);
        }

        let acceleration = self.force * self.inv_mass;
        self.velocity = self.velocity + acceleration * dt;
        self.position = self.position + self.velocity * dt;
        self.force = Vec2::new(0.0, 0.0);
        
        // Angular motion integration (Euler method)
        if self.inv_moment_of_inertia != 0.0 {
            let angular_acceleration = self.torque * self.inv_moment_of_inertia;
            self.angular_velocity += angular_acceleration * dt;
            
            // Clamp angular velocity to prevent extreme spinning
            const MAX_ANGULAR_VELOCITY: f32 = 50.0; // ~8 rotations per second
            self.angular_velocity = self.angular_velocity.clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);
            
            self.rotation += self.angular_velocity * dt;
            
            // Normalize rotation to [0, 2π) to prevent floating point drift
            const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
            self.rotation = self.rotation.rem_euclid(TWO_PI);
            
            self.torque = 0.0;
        }
        
        // Clamp linear velocity to prevent tunneling
        const MAX_VELOCITY: f32 = 1000.0; // Maximum pixels per second
        let speed_sq = self.velocity.length_squared();
        if speed_sq > MAX_VELOCITY * MAX_VELOCITY {
            self.velocity = self.velocity.normalize() * MAX_VELOCITY;
        }
    }

}

#[cfg(test)]
#[path = "tests/body_tests.rs"]
mod body_tests;