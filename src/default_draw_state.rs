use draw_state::state::*;
use DrawState;

static DEFAULT_DRAW_STATE: &'static DrawState =
    &DrawState {
        rasterizer: Rasterizer {
            front_face: FrontFace::CounterClockwise,
            method: RasterMethod::Fill(
                CullFace::Nothing
            ),
            offset: None,
            samples: None
        },
        multi_sample: None,
        scissor: None,
        stencil: None,
        depth: None,
        blend: Some(Blend {
                color: BlendChannel {
                    equation: Equation::Add,
                    source: Factor::ZeroPlus(BlendValue::SourceAlpha),
                    destination: Factor::OneMinus(BlendValue::SourceAlpha),
                },
                alpha: BlendChannel {
                    equation: Equation::Add,
                    source: Factor::One,
                    destination: Factor::One,
                },
            }),
        color_mask: MASK_ALL,
    };

/// Returns a default draw state that does additive blending and no culling.
pub fn default_draw_state() -> &'static DrawState {
    DEFAULT_DRAW_STATE
}
