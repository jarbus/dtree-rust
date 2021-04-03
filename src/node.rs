/// An enum of all the different Shapes a node can take.
/// Currently limited to Rectangles and Circles
#[derive(Clone)]
pub enum Shape {
    Rect,
    Circle
}

/// A node in the dtree. Each node has a unique id, indiciating what the number of nodes generated
/// before it. The root of the tree cannot be deleted, and will always have id 0.
#[derive(Clone)]
pub struct Node {
    /// The position of the node, determined by the view of the graph.
    /// In terms of pixels
    pub render_pos: [f64;2],
    /// shape    - the type of shape to draw
    pub shape: Shape,
    /// color    - color to draw
    pub color: Color,
    /// size     - scalar value indicating size of node in pixels
    pub size: f64,
    /// id       - The id of the node, i.e. the number of unique nodes generated before it
    pub id: usize,
    /// parent   - The id of the parent node
    pub parent: usize,
    /// children - A vector of all children ids
    pub children: Vec<usize>,
}

impl Node{
    /// Renders node at :position: * window_size, where :position: is between 0 and 1
    pub fn draw(&mut self, r: &mut Renderer, position: [f64; 2], text: Option<String>) {
        if let Some(v) = r.c.viewport{
            let x = (position[0] * v.window_size[0]) - (self.size/2.0);
            let y = (position[1] * v.window_size[1]) - (self.size/2.0);

            self.render_pos = [position[0] * v.window_size[0], position[1] * v.window_size[1] ];
            let transform = r.c.transform;


            match self.shape {
                Shape::Rect => graphics::Rectangle::new(self.color).draw([x, y,self.size, self.size], &r.c.draw_state, transform, r.gl),
                Shape::Circle => graphics::CircleArc::new(self.color,10.0,0.0,2.0 * std::f64::consts::PI).draw([x,y,self.size,self.size], &r.c.draw_state, transform, r.gl),
            }
        }
    }

    /// Creates a new node object
    pub fn new(start_shape: Shape, node_id: usize, parent_node: usize) -> Node {
        Node {
            render_pos: [0.0, 0.0],
            shape: start_shape,
            color: WHITE,
            size: 100.0,
            id: node_id,
            parent: parent_node,
            children: vec![],
        }
    }
}
