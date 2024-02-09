#![cfg_attr(target_arch = "spirv", no_std)]
use crabslab::{Id, Slab, SlabItem};
use spirv_std::spirv;

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level0([f32; 4]);

#[cfg(feature = "level0")]
#[spirv(fragment)]
pub fn fragment0(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level0>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level1(Level0, Level0);

#[cfg(feature = "level1")]
#[spirv(fragment)]
pub fn fragment1(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level1>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0 .0;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level2(Level1, Level1);

#[cfg(feature = "level2")]
#[spirv(fragment)]
pub fn fragment2(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level2>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0 .0 .0;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level3(Level2, Level2);

#[cfg(feature = "level3")]
#[spirv(fragment)]
pub fn fragment3(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level3>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0 .0 .0 .0;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level4(Level3, Level3);

#[cfg(feature = "level4")]
#[spirv(fragment)]
pub fn fragment4(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level4>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0 .0 .0 .0 .0;
}

#[derive(Default, Clone, Copy, SlabItem)]
pub struct Level5(Level4, Level4);

#[cfg(feature = "level5")]
#[spirv(fragment)]
pub fn fragment5(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(flat)] id: Id<Level5>,
    frag_color: &mut [f32; 4],
) {
    let val = slab.read(id);
    *frag_color = val.0 .0 .0 .0 .0 .0;
}
