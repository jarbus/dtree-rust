#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub selected: usize,
    pub max_id: usize,
    pub mode: Mode,
}


impl Graph{

    /// Generates and returns a new graph with a single root node of id=0
    pub fn new() -> Graph {
        let mut g: Graph = Graph { nodes: HashMap::new(), selected:0, max_id: 0, mode: Mode::Travel};
        g.reset();
        return g

    }

    /// Renders the graph with the algorithm associated with the parameter :view:
    pub fn draw_view(&mut self, r: &mut Renderer){

        match r.view {
            // ThreeGen displays selected node, parent, and all children
            View::ThreeGen => {
                // Draw selected node in the center and update position
                if let Some(sel) = self.nodes.get_mut(&self.selected){
                    sel.draw(r, [0.5, 0.5], None);
                }
                // Draw each child in a row from left to right
                self.draw_children(r, self.selected, [0.5, 0.5], [0.1,0.9]);

                // Draw a parent if parent != selected, which only
                // occurs for the root
                let parent_id = self.nodes[&self.selected].parent;
                let sel_render_pos = self.nodes[&self.selected].render_pos;
                let par_render_pos = self.nodes[&parent_id].render_pos;
                if parent_id != self.selected{
                    if let Some(parent_node) = self.nodes.get_mut(&parent_id){
                        parent_node.draw(r, [0.5, 0.2],None);
                    }
                }
                piston_window::Line::new(WHITE,10.0)
                    .draw_from_to(sel_render_pos,
                                  par_render_pos,
                                  &r.c.draw_state, r.c.transform,  r.gl);
            },
            View::FiveGen => {},
        }
    }

    /// Draw children of node :node_id: which is located at :position:
    /// between :boundary[0]: and :boundary[1]:, where each boundary
    /// in between [0,1]
    pub fn draw_children(&mut self, r: &mut Renderer, node_id: usize, position: [f64; 2], boundaries: [f64; 2])
    {
        let node = &self.nodes[&node_id].clone();
        if node.children.len() == 0 {return};

        //let maps = vec!["a","s","d","f","g","h","j","l"];
        // if even, draw all nodes evenly spaced centered around position
        let seperation_length = (boundaries[1] - boundaries[0]) / (node.children.len() + 1) as f64;
        for (i, id) in node.children.iter().enumerate() {
                if let Some(child) = self.nodes.get_mut(&id){
                    child.draw(r, [boundaries[0] + ((i+1) as f64 * seperation_length), position[1] + 0.2], None);

                    piston_window::Line::new(WHITE,10.0)
                        .draw_from_to(node.render_pos,
                                    child.render_pos,
                                    &r.c.draw_state, r.c.transform, r.gl);

            }
        }
        // }
    }


    /// Adds child to currently selected node
    pub fn add_child(&mut self){

        // create a new id for the node
        self.max_id = self.max_id + 1;
        let child = Node::new(Shape::Circle, self.max_id, self.selected);
        // Insert child with id=graph.max_id
        self.nodes.insert(self.max_id, child,);
        // Add child_id to selected node's children
        match self.nodes.get_mut(&self.selected){
            Some(sel) => sel.children.push(self.max_id),
            _ => {}
        };
    }

    pub fn edit_node(&mut self){
        // https://docs.piston.rs/gfx_text/gfx_text/

    }
    pub fn toggle_travel_mode(&mut self){
        match self.mode{
            Mode::Travel => self.mode = Mode::None,
            _ => self.mode = Mode::Travel,
        }
    }

    /// Select node in graph
    /// :new_selection: = -1 => select parent
    /// 0 <= new_selection < # of children => select child :new_selection:
    /// else keep current selection
    pub fn select(&mut self, new_selection: i8){
        let new_selection: usize = match new_selection {
            -1  => self.nodes[&self.selected].parent,
            x if x <= self.nodes[&self.selected].children.len() as i8 && x > 0 => self.nodes[&self.selected].children[(x-1) as usize],
            _ => self.selected,
            };

        // Unselect previous node, select new selection
        if let Some(prev_sel) = self.nodes.get_mut(&self.selected){
            prev_sel.color = DEFAULT_NODE_COLOR;
        }
        if let Some(new_sel)  =  self.nodes.get_mut(&new_selection){
            new_sel.color = SELECTED_NODE_COLOR;
        }
        self.selected = new_selection;
    }
    /// Clears graph, creates and selects a new root node
    pub fn reset(&mut self){
        self.nodes.clear();
        self.nodes = HashMap::new();
        let mut root: Node = Node::new(Shape::Circle, 0, 0);
        root.color = SELECTED_NODE_COLOR;
        self.nodes.insert(0, root);
        self.selected = 0;
        self.max_id = 0;
    }
}
