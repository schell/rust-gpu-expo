#![cfg_attr(target_arch = "spirv", no_std)]
use crabslab::{Id, Slab, SlabItem};
use spirv_std::spirv;

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level0 {
    inner: f32,
}

#[cfg(feature = "level0")]
#[spirv(fragment)]
pub fn fragment0(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level0>,
    frag_color: &mut [f32; 4],
) {
    let Level0 { inner } = slab.read(id);
    frag_color[0] = inner;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level1 {
    inner: Level0,
}

#[cfg(feature = "level1")]
#[spirv(fragment)]
pub fn fragment1(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level1>,
    frag_color: &mut [f32; 4],
) {
    let Level1 {
        inner: Level0 { inner },
    } = slab.read(id);
    frag_color[0] = inner;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level2 {
    inner: Level1,
}

#[cfg(feature = "level2")]
#[spirv(fragment)]
pub fn fragment2(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level2>,
    frag_color: &mut [f32; 4],
) {
    let Level2 {
        inner: Level1 {
            inner: Level0 { inner },
        },
    } = slab.read(id);
    frag_color[0] = inner;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level3 {
    inner: Level2,
}

#[cfg(feature = "level3")]
#[spirv(fragment)]
pub fn fragment3(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level3>,
    frag_color: &mut [f32; 4],
) {
    let Level3 {
        inner:
            Level2 {
                inner: Level1 {
                    inner: Level0 { inner },
                },
            },
    } = slab.read(id);
    frag_color[0] = inner;
}
