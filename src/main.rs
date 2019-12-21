use log::*;
use screeps::game;
use stdweb::js;

mod cleanup;
mod creeps;
mod logging;
mod spawn;
mod strategies;
mod units;

use cleanup::run_cleanup;
use creeps::run_creep;
use spawn::run_spawn;

fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    info!(">>> loop starting! cpu: {}", game::cpu::get_used());

    // Spawn some units (maybe)
    for spawn in game::spawns::values() {
        run_spawn(spawn);
    }

    // Run our creeps AI
    for creep in game::creeps::values() {
        run_creep(creep);
    }

    // Be a good citizen
    run_cleanup();

    info!("<<< done! cpu: {}", game::cpu::get_used())
}
