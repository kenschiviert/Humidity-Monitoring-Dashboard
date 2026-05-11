use yew::prelude::*;

// OOP Approach: Define the Sensor structure
struct HumiditySensor {
    value: i32,
}

impl HumiditySensor {
    fn new(val: i32) -> Self {
        Self { value: val }
    }

    // Logic for status messages and colors
    fn get_status(&self) -> (&'static str, &'static str, &'static str) {
        if self.value > 70 {
            ("Udara Terlalu Lembap", "text-red-500", "border-red-500/30 shadow-red-900/10")
        } else if self.value < 40 {
            ("Udara Terlalu Kering", "text-orange-500", "border-orange-500/30 shadow-orange-900/10")
        } else {
            ("Kelembapan Udara Aman", "text-green-500", "border-green-500/30 shadow-green-900/10")
        }
    }

    fn increment(&mut self) {
        self.value = (self.value + 1).min(100);
    }

    fn decrement(&mut self) {
        self.value = (self.value - 1).max(0);
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // State management
    let sensor_state = use_state(|| HumiditySensor::new(50));

    let on_increase = {
        let sensor_state = sensor_state.clone();
        Callback::from(move |_| {
            let mut sensor = HumiditySensor::new(sensor_state.value);
            sensor.increment();
            sensor_state.set(sensor);
        })
    };

    let on_decrease = {
        let sensor_state = sensor_state.clone();
        Callback::from(move |_| {
            let mut sensor = HumiditySensor::new(sensor_state.value);
            sensor.decrement();
            sensor_state.set(sensor);
        })
    };

    let (status_msg, status_color, shadow_style) = sensor_state.get_status();

    html! {
        <main class="min-h-screen bg-[#0F0F0F] text-[#F5F5F5] font-inter flex flex-col items-center justify-center p-4 md:p-8">
            // Card Container with Entrance Animation
            <div class="w-full max-w-sm bg-[#1A1A1A] rounded-[2rem] p-8 border border-white/5 shadow-2xl animate-reveal">
                
                // Header / Status
                <div class="text-center mb-10">
                    <h2 class="text-[10px] uppercase tracking-[0.2em] text-gray-500 mb-2">{"Kondisi Ruangan"}</h2>
                    <p class={classes!("text-sm", "font-medium", "transition-all", "duration-700", status_color)}>
                        {status_msg}
                    </p>
                </div>

                // Gauge Display
                <div class="flex flex-col items-center justify-center mb-10">
                    <div class={classes!(
                        "w-56", "h-56", "rounded-full", "bg-[#141414]", "border-2", "flex", "items-center", "justify-center", "transition-all", "duration-1000",
                        shadow_style
                    )}>
                        <div class="flex flex-col items-center">
                            <span class="text-7xl font-bold tracking-tight">
                                {sensor_state.value}
                            </span>
                            <span class="text-gray-500 text-lg">{"% RH"}</span>
                        </div>
                    </div>
                </div>

                // Controls: Mobile First Grid
                <div class="grid grid-cols-2 gap-4">
                    <button 
                        onclick={on_decrease}
                        aria-label="Decrease Humidity"
                        class="group h-20 rounded-2xl bg-[#222222] hover:bg-[#282828] active:scale-95 transition-all flex items-center justify-center border border-white/5"
                    >
                        <svg class="w-6 h-6 text-gray-400 group-hover:text-white" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path d="M20 12H4" />
                        </svg>
                    </button>

                    <button 
                        onclick={on_increase}
                        aria-label="Increase Humidity"
                        class="group h-20 rounded-2xl bg-[#222222] hover:bg-[#282828] active:scale-95 transition-all flex items-center justify-center border border-white/5"
                    >
                        <svg class="w-6 h-6 text-gray-400 group-hover:text-white" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path d="M12 4v16m8-8H4" />
                        </svg>
                    </button>
                </div>

                <footer class="mt-8 text-center">
                    <p class="text-[9px] text-gray-600 uppercase tracking-widest italic">{"Encrypted Sensor Link Active"}</p>
                </footer>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}