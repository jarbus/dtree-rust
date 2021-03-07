mod node;
use node::Node;

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub selected: usize,
}
impl Graph{
    pub fn new() -> Graph {
        Graph { nodes: vec![Node::new(Shape::Circle,0.5, 0.5); 1], selected:0}
    }
    // Function to change selected node to the node at the new_selection position in the self.nodes
    pub fn select(&mut self, new_selection: i8){
        // Here we handle cases where new_selection refers to an index outside of self.nodes
        // by having the index loop back around to the opposite side of the array
        let new_selection: usize = match new_selection {
            // If goes past the last node in the vector, wrap it to the first
            x if x >= self.nodes.len() as i8 => 0,
            // If it goes below 0, wrap it to the last node in the vector
            x if x < 0 => self.nodes.len() - 1,
            _ => new_selection as usize,
            };

        // Unselect previous node, select new selection
        self.nodes[self.selected as usize].color = DEFAULT_NODE_COLOR;
        self.nodes[new_selection as usize].color = SELECTED_NODE_COLOR;
        self.selected = new_selection;
    }
}
