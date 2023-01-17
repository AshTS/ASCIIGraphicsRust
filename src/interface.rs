use sdl2::{render::Canvas, EventPump};

pub struct GameInterface {
    pub context: sdl2::Sdl,
    pub canvas: Canvas<sdl2::video::Window>,

    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: sdl2::TimerSubsystem,

    pub event_pump: EventPump,
}

impl GameInterface {
    /// Construct a new GameInterface object using sdl2
    pub fn new(default_size: (usize, usize)) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Engine", 
                (default_size.0) as u32,
                (default_size.1) as u32)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

        let timer_subsystem = sdl_context.timer()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            context: sdl_context,
            canvas,
            video_subsystem,
            timer_subsystem,
            event_pump
        })
    }
}