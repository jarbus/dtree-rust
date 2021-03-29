pub struct Renderer<'a> {

    pub c: graphics::context::Context,
    pub gl: &'a mut GlGraphics,
    pub g2d: &'a  G2d,
    pub glyphs: Glyphs,
    pub view: View,
}
