use embedded_graphics::{
    geometry::{Point, Size},
    mono_font::{iso_8859_9::FONT_8X13, MonoTextStyle},
    pixelcolor::{BinaryColor, PixelColor},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    text::Text,
    Drawable,
};

use crate::Widget;

impl<'a, C> Drawable for Widget<'a, C>
where
    C: PixelColor + From<BinaryColor>,
{
    type Color = C;
    type Output = ();
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        match self.widget {
            crate::WidgetList::Battery(min, max, value) => {
                let point2_top_left = Point::new(
                    self.position.x,
                    self.position.y + (self.size.height as f32 * 0.25) as i32,
                );
                // let point1_top_left = Point::new(
                //     self.position.x + (self.size.width as f32 * 1.0 / 3.0) as i32,
                //     self.position.y,
                // );
                Rectangle::new(
                    point2_top_left,
                    Size::new(
                        self.size.width,
                        self.size.height - (self.size.height as f32 * 0.25) as u32,
                    ),
                )
                .into_styled(PrimitiveStyle::with_stroke(self.color, 1))
                .draw(target)?;
                if value.output_voltage(5.0) < min {};
                let percentage = (max as f32 - value.output_voltage(5.0)) * 100.0;
                Text::with_alignment(
                    format!("{:.2}%", percentage).as_str(),
                    point2_top_left,
                    MonoTextStyle::new(&FONT_8X13, self.color),
                    embedded_graphics::text::Alignment::Left,
                )
                .draw(target)?;
                // Text::new(
                //     format!("{:.2}", percentage).as_str(),
                //     self.position,
                //     MonoTextStyle::new(&FONT_4X6, self.color),
                // )
                // .draw(target)?;
                // Rectangle::new(poin, size)
            }
        }
        Ok(())
    }
}
