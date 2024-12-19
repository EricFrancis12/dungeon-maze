use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

pub fn make_wall_with_window_gap_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-2.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
            [-0.5, 0.0, 0.0],
            [2.0, 2.0, 1.0],
            [-2.0, 2.0, 1.0],
            [0.5, -0.0, 1.0],
            [2.0, -2.0, -0.0],
            [0.5, -2.0, 1.0],
            [0.5, -2.0, -0.0],
            [-2.0, -2.0, 1.0],
            [-0.5, -2.0, -0.0],
            [-0.5, -2.0, 1.0],
            [2.0, 2.0, 0.0],
            [2.0, -2.0, 1.0],
            [2.0, -2.0, -0.0],
            [-2.0, -2.0, -0.0],
            [-2.0, 2.0, 1.0],
            [-2.0, 2.0, 0.0],
            [-2.0, 2.0, 0.0],
            [2.0, 2.0, 1.0],
            [2.0, 2.0, 0.0],
            [-0.5, 0.0, 0.0],
            [-0.5, -2.0, 1.0],
            [-0.5, -2.0, -0.0],
            [0.5, -0.0, 1.0],
            [0.5, -2.0, -0.0],
            [0.5, -2.0, 1.0],
            [-0.5, 0.0, 0.0],
            [0.5, -0.0, 1.0],
            [-0.5, -0.0, 1.0],
            [-0.5, -1.0, 1.0],
            [-0.5, -2.0, 0.0],
            [-0.5, -2.0, 1.0],
            [-0.5, -1.0, 0.0],
            [0.5, -2.0, 0.0],
            [-0.5, -2.0, 0.0],
            [0.5, -2.0, 0.0],
            [0.5, -1.0, 1.0],
            [0.5, -2.0, 1.0],
            [0.5, -1.0, 1.0],
            [-0.5, -2.0, 1.0],
            [0.5, -2.0, 1.0],
            [0.5, -2.0, 0.0],
            [-0.5, -2.0, 1.0],
            [-0.5, -2.0, 0.0],
            [-0.5, -1.0, 0.0],
            [0.5, -1.0, 1.0],
            [0.5, -1.0, 0.0],
            [-0.5, -2.0, -0.0],
            [-2.0, -2.0, -0.0],
            [-0.5, 0.0, 0.0],
            [-2.0, -2.0, -0.0],
            [-2.0, 2.0, 0.0],
            [-0.5, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [2.0, -2.0, -0.0],
            [0.5, 0.0, 0.0],
            [2.0, -2.0, -0.0],
            [0.5, -2.0, -0.0],
            [0.5, 0.0, 0.0],
            [2.0, 2.0, 0.0],
            [0.5, 0.0, 0.0],
            [-0.5, 0.0, 0.0],
            [0.5, -0.0, 1.0],
            [0.5, -2.0, 1.0],
            [2.0, -2.0, 1.0],
            [-2.0, 2.0, 1.0],
            [-2.0, -2.0, 1.0],
            [-0.5, -0.0, 1.0],
            [-2.0, -2.0, 1.0],
            [-0.5, -2.0, 1.0],
            [-0.5, -0.0, 1.0],
            [0.5, -0.0, 1.0],
            [2.0, -2.0, 1.0],
            [2.0, 2.0, 1.0],
            [-2.0, 2.0, 1.0],
            [-0.5, -0.0, 1.0],
            [0.5, -0.0, 1.0],
            [2.0, -2.0, -0.0],
            [2.0, -2.0, 1.0],
            [0.5, -2.0, 1.0],
            [-2.0, -2.0, 1.0],
            [-2.0, -2.0, -0.0],
            [-0.5, -2.0, -0.0],
            [2.0, 2.0, 0.0],
            [2.0, 2.0, 1.0],
            [2.0, -2.0, 1.0],
            [-2.0, -2.0, -0.0],
            [-2.0, -2.0, 1.0],
            [-2.0, 2.0, 1.0],
            [-2.0, 2.0, 0.0],
            [-2.0, 2.0, 1.0],
            [2.0, 2.0, 1.0],
            [-0.5, 0.0, 0.0],
            [-0.5, -0.0, 1.0],
            [-0.5, -2.0, 1.0],
            [0.5, -0.0, 1.0],
            [0.5, 0.0, 0.0],
            [0.5, -2.0, -0.0],
            [-0.5, 0.0, 0.0],
            [0.5, 0.0, 0.0],
            [0.5, -0.0, 1.0],
            [-0.5, -1.0, 1.0],
            [-0.5, -1.0, 0.0],
            [-0.5, -2.0, 0.0],
            [-0.5, -1.0, 0.0],
            [0.5, -1.0, 0.0],
            [0.5, -2.0, 0.0],
            [0.5, -2.0, 0.0],
            [0.5, -1.0, 0.0],
            [0.5, -1.0, 1.0],
            [0.5, -1.0, 1.0],
            [-0.5, -1.0, 1.0],
            [-0.5, -2.0, 1.0],
            [0.5, -2.0, 0.0],
            [0.5, -2.0, 1.0],
            [-0.5, -2.0, 1.0],
            [-0.5, -1.0, 0.0],
            [-0.5, -1.0, 1.0],
            [0.5, -1.0, 1.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 1.0],
            [1.0, 1.0],
            [0.375, 0.5],
            [1.0, 1.0],
            [0.0, 1.0],
            [0.625, 0.5],
            [1.0, 0.0],
            [0.625, 0.0],
            [0.625, 0.0],
            [0.0, 0.0],
            [0.375, 0.0],
            [0.375, 0.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 1.0],
            [0.625, 0.179821],
            [0.375, 0.125],
            [0.375, 0.179821],
            [0.625, 0.625],
            [0.375, 0.570178986],
            [0.375, 0.625],
            [0.875, 0.5],
            [0.625, 0.750],
            [0.875, 0.5],
            [0.625, 0.0],
            [0.375, 0.250],
            [0.375, 0.0],
            [0.625, 0.250],
            [0.375, 0.5],
            [0.375, 0.250],
            [0.375, 0.5],
            [0.625, 0.750],
            [0.375, 0.750],
            [0.625, 0.750],
            [0.375, 1.0],
            [0.375, 0.750],
            [0.375, 0.5],
            [0.125, 0.750],
            [0.125, 0.5],
            [0.875, 0.5],
            [0.625, 0.750],
            [0.625, 0.5],
            [0.375, 0.0],
            [0.0, 0.0],
            [0.375, 0.5],
            [0.0, 0.0],
            [0.0, 1.0],
            [0.375, 0.5],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.625, 0.5],
            [1.0, 0.0],
            [0.625, 0.0],
            [0.625, 0.5],
            [1.0, 1.0],
            [0.625, 0.5],
            [0.375, 0.5],
            [0.625, 0.5],
            [0.625, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [0.0, 0.0],
            [0.375, 0.5],
            [0.0, 0.0],
            [0.375, 0.0],
            [0.375, 0.5],
            [0.625, 0.5],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            [0.375, 0.5],
            [0.625, 0.5],
            [1.0, 0.0],
            [1.0, 0.0],
            [0.625, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.375, 0.0],
            [1.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [0.625, 0.179821],
            [0.625, 0.125],
            [0.375, 0.125],
            [0.625, 0.625],
            [0.625, 0.570178986],
            [0.375, 0.570178986],
            [0.875, 0.5],
            [0.625, 0.750],
            [0.625, 0.750],
            [0.625, 0.0],
            [0.625, 0.250],
            [0.375, 0.250],
            [0.625, 0.250],
            [0.625, 0.5],
            [0.375, 0.5],
            [0.375, 0.5],
            [0.625, 0.5],
            [0.625, 0.750],
            [0.625, 0.750],
            [0.625, 1.0],
            [0.375, 1.0],
            [0.375, 0.5],
            [0.375, 0.750],
            [0.125, 0.750],
            [0.875, 0.5],
            [0.875, 0.750],
            [0.625, 0.750],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-1.0, -0.0, -0.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [-0.0, -0.0, -1.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [1.0, -0.0, -0.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -0.0, 1.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, -1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
            [-0.0, 1.0, -0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119,
    ]))
}
