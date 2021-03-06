pub fn compute() -> stretch::result::Layout {
    stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                min_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![
                &stretch::node::Node::new(
                    stretch::style::Style {
                        flex_grow: 1f32,
                        flex_basis: stretch::style::Dimension::Points(200f32),
                        ..Default::default()
                    },
                    vec![],
                ),
                &stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            height: stretch::style::Dimension::Points(100f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap()
}
