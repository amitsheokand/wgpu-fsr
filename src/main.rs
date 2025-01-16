use wgpu_fsr::run;

fn main() {
    pollster::block_on(run());
}
