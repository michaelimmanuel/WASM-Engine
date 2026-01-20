# WebAssembly Physics Engine - JavaScript API Documentation

This document describes how to use the physics engine from JavaScript in a web browser.

## Installation & Setup

### 1. Build the WASM Module

```bash
wasm-pack build --target web
```

This generates the compiled WASM files in the `pkg/` directory.

### 2. Import in HTML

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Physics Engine Demo</title>
</head>
<body>
    <canvas id="canvas" width="800" height="600"></canvas>
    
    <script type="module">
        import init, { World } from './pkg/wasm_engine.js';
        
        async function start() {
            // Initialize WASM module
            await init();
            
            // Create physics world
            const world = new World();
            
            // Your game code here...
        }
        
        start();
    </script>
</body>
</html>
```

---

---

## API Reference

### `World` Class

The main physics simulation container.

---

### Constructor

#### `new World()`

Creates a new physics world.

```javascript
const world = new World();
```

**Returns:** A new `World` instance

---

### Creating Bodies

#### `create_circle(mass, x, y)`

Creates a circular body (sphere/ball).

```javascript
const bodyIndex = world.create_circle(mass, x, y);
```

**Parameters:**
- `mass: number` - Mass of the body (use `0` for static/immovable objects)
- `x: number` - Initial X position
- `y: number` - Initial Y position

**Returns:** `number` - Index of the created body (use this to reference the body later)

**Default Properties:**
- Shape: Circle with radius 20.0
- Gravity: Enabled
- Restitution: 0.3 (moderate bounciness)

**Example:**
```javascript
// Create a dynamic ball
const ball = world.create_circle(1.0, 400, 100);

// Create a static (immovable) object
const staticBody = world.create_circle(0, 300, 500);
```

---

#### `create_box(mass, x, y, width, height)`

Creates a rectangular body (box/platform).

```javascript
const boxIndex = world.create_box(mass, x, y, width, height);
```

**Parameters:**
- `mass: number` - Mass of the body (use `0` for static objects)
- `x: number` - Initial X position (center of box)
- `y: number` - Initial Y position (center of box)
- `width: number` - Width of the box
- `height: number` - Height of the box

**Returns:** `number` - Index of the created body

**Example:**
```javascript
// Create a floor platform
const floor = world.create_box(0, 400, 550, 800, 50);

// Create a dynamic box
const crate = world.create_box(2.0, 400, 200, 60, 60);
```

---

### Physics Simulation

#### `step(dt)`

Advances the physics simulation by one time step.

```javascript
world.step(dt);
```

**Parameters:**
- `dt: number` - Delta time in seconds (e.g., `1/60` for 60 FPS)

**What it does:**
1. Integrates positions (applies velocities and gravity)
2. Detects collisions between all bodies
3. Resolves collisions (4 iterations for stability)

**Example:**
```javascript
function gameLoop() {
    const dt = 1 / 60;  // 60 FPS
    world.step(dt);
    
    // Render your scene...
    
    requestAnimationFrame(gameLoop);
}

requestAnimationFrame(gameLoop);
```

---

### Position & Velocity

#### `get_body_position_x(index)`

Gets the X position of a body.

```javascript
const x = world.get_body_position_x(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - X coordinate

---

#### `get_body_position_y(index)`

Gets the Y position of a body.

```javascript
const y = world.get_body_position_y(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - Y coordinate

---

#### `set_body_position(index, x, y)`

Sets the position of a body.

```javascript
world.set_body_position(bodyIndex, x, y);
```

**Parameters:**
- `index: number` - Body index
- `x: number` - New X position
- `y: number` - New Y position

**Warning:** Avoid teleporting bodies through other bodies, as this may cause unexpected collisions.

---

#### `set_body_velocity(index, vx, vy)`

Sets the velocity of a body.

```javascript
world.set_body_velocity(bodyIndex, vx, vy);
```

**Parameters:**
- `index: number` - Body index
- `vx: number` - Velocity X component (pixels/second)
- `vy: number` - Velocity Y component (pixels/second)

**Example:**
```javascript
// Make ball move right at 200 px/s
world.set_body_velocity(ball, 200, 0);

// Launch upward
world.set_body_velocity(ball, 0, -300);
```

---

#### `get_body_velocity_x(index)`

Gets the X velocity of a body.

```javascript
const vx = world.get_body_velocity_x(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - Velocity X component

---

#### `get_body_velocity_y(index)`

Gets the Y velocity of a body.

```javascript
const vy = world.get_body_velocity_y(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - Velocity Y component

---

### Body Properties

#### `set_body_gravity(index, enabled)`

Enables or disables gravity for a body.

```javascript
world.set_body_gravity(bodyIndex, enabled);
```

**Parameters:**
- `index: number` - Body index
- `enabled: boolean` - `true` to enable gravity, `false` to disable

**Default:** Gravity is enabled by default (9.8 m/s² downward)

**Example:**
```javascript
// Disable gravity for a floating platform
world.set_body_gravity(platform, false);

// Re-enable gravity
world.set_body_gravity(ball, true);
```

---

#### `set_body_restitution(index, restitution)`

Sets the bounciness (restitution) of a body.

```javascript
world.set_body_restitution(bodyIndex, restitution);
```

**Parameters:**
- `index: number` - Body index
- `restitution: number` - Bounciness coefficient (0.0 - 1.0)
  - `0.0` = No bounce (perfectly inelastic)
  - `0.5` = Moderate bounce
  - `1.0` = Perfect bounce (no energy loss)

**Default:** 0.3

**Example:**
```javascript
// Very bouncy ball
world.set_body_restitution(ball, 0.9);

// No bounce (sticks on impact)
world.set_body_restitution(box, 0.0);
```

---

### Collision Detection

#### `collides(i, j)`

Quick check if two bodies are colliding.

```javascript
const isColliding = world.collides(bodyIndex1, bodyIndex2);
```

**Parameters:**
- `i: number` - First body index
- `j: number` - Second body index

**Returns:** `boolean` - `true` if bodies are overlapping, `false` otherwise

**Example:**
```javascript
if (world.collides(player, enemy)) {
    console.log("Player hit enemy!");
}
```

---

#### `get_collision_penetration(i, j)`

Gets the penetration depth of a collision.

```javascript
const depth = world.get_collision_penetration(bodyIndex1, bodyIndex2);
```

**Parameters:**
- `i: number` - First body index
- `j: number` - Second body index

**Returns:** `number` - Penetration depth (0 if not colliding)

---

#### `get_collision_normal_x(i, j)`

Gets the X component of the collision normal.

```javascript
const normalX = world.get_collision_normal_x(bodyIndex1, bodyIndex2);
```

**Parameters:**
- `i: number` - First body index
- `j: number` - Second body index

**Returns:** `number` - Normal X component (0 if not colliding)

**Note:** The normal points from body `i` toward body `j`.

---

#### `get_collision_normal_y(i, j)`

Gets the Y component of the collision normal.

```javascript
const normalY = world.get_collision_normal_y(bodyIndex1, bodyIndex2);
```

**Parameters:**
- `i: number` - First body index
- `j: number` - Second body index

**Returns:** `number` - Normal Y component (0 if not colliding)

---

### Shape Information

#### `is_body_circle(index)`

Checks if a body is a circle.

```javascript
const isCircle = world.is_body_circle(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `boolean` - `true` if circle, `false` if box

---

#### `is_body_box(index)`

Checks if a body is a box.

```javascript
const isBox = world.is_body_box(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `boolean` - `true` if box, `false` if circle

---

#### `get_body_width(index)`

Gets the width of a body.

```javascript
const width = world.get_body_width(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - Width (for boxes) or diameter (for circles)

---

#### `get_body_height(index)`

Gets the height of a body.

```javascript
const height = world.get_body_height(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `number` - Height (for boxes) or diameter (for circles)

---

#### `is_body_static(index)`

Checks if a body is static (immovable).

```javascript
const isStatic = world.is_body_static(bodyIndex);
```

**Parameters:**
- `index: number` - Body index

**Returns:** `boolean` - `true` if static (mass = 0), `false` if dynamic

---

### World Information

#### `bodies_count()`

Gets the total number of bodies in the world.

```javascript
const count = world.bodies_count();
```

**Returns:** `number` - Total body count

**Example:**
```javascript
for (let i = 0; i < world.bodies_count(); i++) {
    const x = world.get_body_position_x(i);
    const y = world.get_body_position_y(i);
    // Render body at (x, y)...
}
```

---

## Complete Example

```javascript
import init, { World } from './pkg/wasm_engine.js';

async function main() {
    // Initialize WASM
    await init();
    
    // Setup canvas
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    
    // Create physics world
    const world = new World();
    
    // Create floor
    const floor = world.create_box(0, 400, 550, 800, 50);
    world.set_body_restitution(floor, 0.8);
    
    // Create walls
    const leftWall = world.create_box(0, 25, 300, 50, 600);
    const rightWall = world.create_box(0, 775, 300, 50, 600);
    
    // Create a bouncy ball
    const ball = world.create_circle(1.0, 400, 100);
    world.set_body_restitution(ball, 0.9);
    world.set_body_velocity(ball, 100, 0);
    
    // Create some boxes
    const box1 = world.create_box(1.5, 300, 200, 60, 60);
    const box2 = world.create_box(1.5, 500, 200, 60, 60);
    
    // Game loop
    function gameLoop() {
        // Clear canvas
        ctx.fillStyle = '#f0f0f0';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Update physics
        world.step(1 / 60);
        
        // Render all bodies
        for (let i = 0; i < world.bodies_count(); i++) {
            const x = world.get_body_position_x(i);
            const y = world.get_body_position_y(i);
            
            ctx.fillStyle = world.is_body_static(i) ? '#888' : '#4a90e2';
            
            if (world.is_body_circle(i)) {
                const radius = world.get_body_width(i) / 2;
                ctx.beginPath();
                ctx.arc(x, y, radius, 0, Math.PI * 2);
                ctx.fill();
            } else {
                const w = world.get_body_width(i);
                const h = world.get_body_height(i);
                ctx.fillRect(x - w/2, y - h/2, w, h);
            }
        }
        
        requestAnimationFrame(gameLoop);
    }
    
    gameLoop();
}

main();
```

---

## Usage Tips

### Performance

1. **Minimize body count** - Collision detection is O(n²)
2. **Use static bodies** - Set mass to 0 for immovable objects (walls, floors)
3. **Adjust time step** - Lower values (e.g., 1/120) = more accurate but slower

### Stability

1. **Keep velocities reasonable** - Very high speeds may cause tunneling
2. **Use appropriate sizes** - Very small or large bodies may be unstable
3. **Don't teleport** - Gradually move bodies instead of instant position changes

### Coordinate System

- **Origin:** Top-left corner (0, 0)
- **X-axis:** Right is positive
- **Y-axis:** Down is positive (screen coordinates)
- **Gravity:** Points downward (+Y direction)

### Common Patterns

#### Click to Spawn

```javascript
canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const ball = world.create_circle(1.0, x, y);
    world.set_body_restitution(ball, 0.8);
});
```

#### Apply Force on Keypress

```javascript
document.addEventListener('keydown', (e) => {
    if (e.key === ' ') {
        // Jump
        const vy = world.get_body_velocity_y(player);
        if (Math.abs(vy) < 10) { // Only if on ground
            world.set_body_velocity(player, 0, -400);
        }
    }
});
```

#### Collision Callbacks

```javascript
function checkCollisions() {
    if (world.collides(player, enemy)) {
        console.log("Game Over!");
        // Handle player-enemy collision
    }
    
    if (world.collides(player, powerup)) {
        console.log("Collected powerup!");
        // Handle collection logic
    }
}
```

---

## Building for Production

### Optimize Build

```bash
wasm-pack build --target web --release
```

The `--release` flag enables optimizations for smaller file size and better performance.

### File Structure

After building, your project should look like:

```
your-project/
├── index.html
├── pkg/
│   ├── wasm_engine.js
│   ├── wasm_engine_bg.wasm
│   ├── wasm_engine.d.ts
│   └── package.json
└── src/
    └── (Rust source files)
```

---

## TypeScript Support

The WASM module includes TypeScript definitions:

```typescript
import init, { World } from './pkg/wasm_engine.js';

const world: World = new World();
const ball: number = world.create_circle(1.0, 400, 300);
```

---

## Troubleshooting

### WASM not loading

**Problem:** `fetch` errors or WASM initialization fails

**Solution:** Serve files from a local web server, not `file://`

```bash
# Python 3
python -m http.server 8000

# Node.js
npx serve
```

### Bodies falling through floor

**Problem:** Fast-moving objects tunnel through thin obstacles

**Solutions:**
1. Increase floor thickness
2. Decrease time step (e.g., 1/120 instead of 1/60)
3. Limit maximum velocities

### Jittery collisions

**Problem:** Bodies vibrate when stacked

**Solution:** The engine already includes:
- Penetration slop (0.005)
- 80% positional correction
- 4 collision iterations per step

---

## See Also

- [COLLISION_DETECTION_NOTES.md](COLLISION_DETECTION_NOTES.md) - Theory and algorithms
- [GitHub Repository](https://github.com/your-repo) - Source code
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) - Rust ↔ JS interop

Represents a collision contact point between two bodies.

```rust
pub struct Contact {
    pub normal: Vec2,
    pub penetration: f32,
    pub body_a_index: usize,
    pub body_b_index: usize,
}
```

#### Fields

- **`normal: Vec2`** - The collision normal vector pointing from body A toward body B (unit length)
- **`penetration: f32`** - The depth of penetration/overlap between the two bodies
- **`body_a_index: usize`** - Index of the first body in the collision
- **`body_b_index: usize`** - Index of the second body in the collision

#### Methods

##### `new`

Creates a new contact.

```rust
pub fn new(
    normal: Vec2,
    penetration: f32,
    body_a_index: usize,
    body_b_index: usize
) -> Self
```

**Parameters:**
- `normal` - The collision normal vector (should be normalized)
- `penetration` - The penetration depth
- `body_a_index` - Index of body A
- `body_b_index` - Index of body B

**Returns:** A new `Contact` instance

**Example:**
```rust
let contact = Contact::new(
    Vec2::new(1.0, 0.0),  // Normal pointing right
    0.5,                   // 0.5 units of penetration
    0,                     // Body A index
    1                      // Body B index
);
```

---

## Functions

### `overlaps`

Quick boolean check for whether two bodies are overlapping.

```rust
pub fn overlaps(a: &Body, b: &Body) -> bool
```

**Parameters:**
- `a` - Reference to the first body
- `b` - Reference to the second body

**Returns:** `true` if the bodies overlap, `false` otherwise

**Supported Shape Combinations:**
- Circle vs Circle
- AABB vs AABB
- Circle vs AABB
- AABB vs Circle

**Example:**
```rust
let circle = Body::new(1.0, Vec2::new(0.0, 0.0), Shape::Circle { radius: 1.0 });
let box_body = Body::new(1.0, Vec2::new(1.5, 0.0), Shape::Box { width: 2.0, height: 2.0 });

if overlaps(&circle, &box_body) {
    println!("Bodies are overlapping!");
}
```

**Performance:** This is faster than `detect_collision` as it only checks for overlap without computing contact details.

---

### `detect_collision`

Detects collision between two bodies and returns detailed contact information.

```rust
pub fn detect_collision(
    a: &Body,
    b: &Body,
    index_a: usize,
    index_b: usize
) -> Option<Contact>
```

**Parameters:**
- `a` - Reference to the first body
- `b` - Reference to the second body
- `index_a` - Index/identifier for body A
- `index_b` - Index/identifier for body B

**Returns:**
- `Some(Contact)` - If bodies are colliding, returns contact information
- `None` - If bodies are not colliding

**Supported Shape Combinations:**
- Circle vs Circle
- AABB vs AABB
- Circle vs AABB
- AABB vs Circle

**Normal Convention:** The collision normal always points from body A toward body B.

**Example:**
```rust
let circle = Body::new(1.0, Vec2::new(0.0, 0.0), Shape::Circle { radius: 1.0 });
let wall = Body::new(0.0, Vec2::new(3.0, 0.0), Shape::Box { width: 2.0, height: 4.0 });

if let Some(contact) = detect_collision(&circle, &wall, 0, 1) {
    println!("Collision detected!");
    println!("Normal: {:?}", contact.normal);
    println!("Penetration: {}", contact.penetration);
}
```

**Algorithm Details:**

#### Circle vs Circle
Uses distance comparison:
- Collision when `distance < radius1 + radius2`
- Normal points from circle A center toward circle B center
- Penetration = `(radius1 + radius2) - distance`

#### AABB vs AABB
Uses Separating Axis Theorem (SAT):
- Tests for overlap on X and Y axes
- Collision if overlapping on both axes
- Normal uses axis of least penetration
- Penetration = minimum overlap amount

#### Circle vs AABB
Uses closest point method:
1. Find closest point on AABB surface to circle center
2. Calculate distance from circle center to closest point
3. If distance ≤ radius → collision
4. Two cases:
   - **Circle outside**: Normal from circle toward closest point
   - **Circle inside**: Normal toward nearest edge

---

### `resolve_collision`

Resolves a collision by applying impulses and position correction to the bodies.

```rust
pub fn resolve_collision(
    a: &mut Body,
    b: &mut Body,
    contact: &Contact
)
```

**Parameters:**
- `a` - Mutable reference to body A
- `b` - Mutable reference to body B
- `contact` - The contact information from `detect_collision`

**Effects:**
- Modifies velocities of both bodies based on impulse response
- Applies positional correction to prevent sinking
- Does nothing if both bodies are static (inv_mass = 0)

**Resolution Steps:**

1. **Impulse Calculation:**
   - Uses relative velocity along collision normal
   - Applies restitution (bounciness) coefficient
   - Distributes impulse based on inverse masses

2. **Positional Correction:**
   - Prevents objects from sinking into each other
   - Uses penetration depth with slop tolerance
   - Applies 80% correction per frame to reduce jitter

**Constants:**
- `PERCENT = 0.8` - Percentage of penetration to correct (80%)
- `SLOP = 0.005` - Small allowed overlap to reduce jitter

**Example:**
```rust
let mut circle = Body::new(1.0, Vec2::new(0.0, 0.0), Shape::Circle { radius: 1.0 });
circle.velocity = Vec2::new(5.0, 0.0);

let mut wall = Body::new(0.0, Vec2::new(3.0, 0.0), Shape::Box { width: 2.0, height: 4.0 });
wall.restitution = 0.8; // Bouncy wall

if let Some(contact) = detect_collision(&circle, &wall, 0, 1) {
    resolve_collision(&mut circle, &mut wall, &contact);
    // Circle now bounces off the wall
}
```

**Physics Properties Used:**
- `Body.velocity` - Linear velocity (modified)
- `Body.position` - Position (modified for correction)
- `Body.inv_mass` - Inverse mass (0 = static/infinite mass)
- `Body.restitution` - Bounciness coefficient (0.0 = no bounce, 1.0 = perfect bounce)

**Special Cases:**

1. **Static Objects (inv_mass = 0):**
   - If both bodies are static, no resolution occurs
   - If one is static, only the dynamic body is affected

2. **Separating Velocities:**
   - If bodies are already moving apart, no impulse is applied
   - Checked by: `relative_velocity • normal > 0`

3. **Restitution:**
   - Average of both bodies' restitution values
   - Controls how "bouncy" the collision is

---

## Usage Example: Complete Physics Step

```rust
use wasm_engine::physics::{Body, collision};
use wasm_engine::math::Vec2;

// Create bodies
let mut ball = Body::new(1.0, Vec2::new(0.0, 5.0), Shape::Circle { radius: 1.0 });
ball.velocity = Vec2::new(0.0, -10.0); // Falling down
ball.restitution = 0.7; // Somewhat bouncy

let mut floor = Body::new(0.0, Vec2::new(0.0, 0.0), Shape::Box { width: 20.0, height: 1.0 });
floor.restitution = 0.9; // Very bouncy

// Physics update loop
let dt = 1.0 / 60.0; // 60 FPS

// 1. Update positions
ball.update(dt);
floor.update(dt);

// 2. Detect collision
if let Some(contact) = collision::detect_collision(&ball, &floor, 0, 1) {
    // 3. Resolve collision
    collision::resolve_collision(&mut ball, &mut floor, &contact);
    
    println!("Ball bounced! New velocity: {:?}", ball.velocity);
}
```

---

## Usage Example: Broad Phase Optimization

For many bodies, use `overlaps` first as a broad phase check:

```rust
let mut bodies = vec![/* ... many bodies ... */];

for i in 0..bodies.len() {
    for j in (i + 1)..bodies.len() {
        // Quick broad phase check
        if collision::overlaps(&bodies[i], &bodies[j]) {
            // Detailed collision detection
            if let Some(contact) = collision::detect_collision(
                &bodies[i],
                &bodies[j],
                i,
                j
            ) {
                // Store contact for later resolution
                contacts.push(contact);
            }
        }
    }
}

// Resolve all contacts
for contact in contacts {
    collision::resolve_collision(
        &mut bodies[contact.body_a_index],
        &mut bodies[contact.body_b_index],
        &contact
    );
}
```

---

## Important Notes

### Normal Direction Convention

**Critical:** The collision normal always points **from body A toward body B**.

This convention must be maintained for correct impulse resolution:
- Body A receives impulse in the `-normal` direction
- Body B receives impulse in the `+normal` direction

### Performance Considerations

1. **Overlaps vs Detect Collision:**
   - Use `overlaps()` for quick boolean checks
   - Use `detect_collision()` only when contact details are needed

2. **Shape Complexity:**
   - Circle vs Circle: Fastest
   - AABB vs AABB: Fast
   - Circle vs AABB: Moderate (requires closest point calculation)

3. **Broad Phase:**
   - Consider spatial partitioning for many bodies
   - Current implementation is O(n²) for n bodies

### Coordinate System

- **Origin:** Center of the window/world
- **X-axis:** Right is positive
- **Y-axis:** Down is positive (typical screen coordinates)
- **Positions:** Body positions are at their centers

### Limitations

1. **No Rotation:** Current AABBs cannot rotate (axis-aligned only)
2. **No Continuous Collision:** Fast-moving objects may tunnel through thin obstacles
3. **No Friction:** Objects slide without resistance
4. **Sequential Resolution:** Multiple simultaneous contacts may have order-dependent behavior

---

## Related Types

### `Body`

```rust
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shape: Shape,
    pub inv_mass: f32,     // 0 = static/infinite mass
    pub restitution: f32,  // 0.0 - 1.0 bounciness
    // ...
}
```

### `Shape`

```rust
pub enum Shape {
    Circle { radius: f32 },
    Box { width: f32, height: f32 },
}
```

### `Vec2`

```rust
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

**Key Methods:**
- `dot(other: Vec2) -> f32` - Dot product
- `normalize() -> Vec2` - Returns unit vector
- Arithmetic: `+`, `-`, `*`, `/` operators supported

---

## See Also

- `physics::body` - Body physics properties
- `physics::world` - World simulation container
- `math::vec2` - Vector mathematics
