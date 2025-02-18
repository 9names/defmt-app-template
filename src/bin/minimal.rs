#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use {{crate_name}} as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = some_hal::pac, // TODO: Replace `some_hal::pac` with the path to the PAC
    dispatchers = [FreeInterrupt1, ...] // TODO: Replace the `FreeInterrupt1, ...` with free interrupt vectors if software tasks are used
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        // TODO setup monotonic if used
        // let token = systick_monotonics::create_systick_token!();
        // rtic_monotonics::Systick::new(cx.core.SYST, sysclk, token);


        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task]
    async fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }
}
