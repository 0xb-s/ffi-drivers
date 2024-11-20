#![no_std]
extern crate alloc;
use core::convert::Infallible;
use core::slice;
use drivers::drivers::apa102::Apa102;
use drivers::drivers::LedDriver;
use embedded_hal_async::spi::{ErrorType, SpiBus};
use smart_leds::RGB8;

pub struct MockSpiBus;


impl ErrorType for MockSpiBus {
    type Error = Infallible;
}

impl SpiBus<u8> for &mut MockSpiBus {
    async fn read(&mut self, _data: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn write(&mut self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transfer(&mut self, _write: &mut [u8], _read: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transfer_in_place(&mut self, _data: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[repr(C)]
pub struct CRGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[no_mangle]
pub async extern "C" fn apa_write_direct(
    spi: &mut MockSpiBus,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    num_leds: usize,
    colors_ptr: *const CRGB8,
    colors_len: usize,
) -> i32 {
    if buffer_ptr.is_null() || colors_ptr.is_null() {
        return -1;
    }

    unsafe {
        let buffer = slice::from_raw_parts_mut(buffer_ptr, buffer_len);
        let colors = slice::from_raw_parts(colors_ptr, colors_len);

        let start_frame_size = 4;
        let led_frame_size = 4;
        let end_frame_size = (num_leds + 15) / 16;
        let total_size = start_frame_size + (num_leds * led_frame_size) + end_frame_size;

        if buffer.len() < total_size {
            return -1;
        }

        let mut apa = Apa102::new(spi, num_leds, buffer);

        let rgb_colors: alloc::vec::Vec<RGB8> = colors
            .iter()
            .map(|color| RGB8 {
                r: color.r,
                g: color.g,
                b: color.b,
            })
            .collect();

        match apa.write(&rgb_colors).await {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}
