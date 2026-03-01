//! This module contains all the init code for the rp2040
//! board. The board is a little tricky to boot - since it uses
//! a 2-stage boot process (chapter 2.7).
//!
//! This module also contains post-boot-2-code, which clears
//! BSS, copies data to RAM, and sets up the vector table.

pub mod boot2;
pub mod init;
