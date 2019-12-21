
use log::*;
use stdweb::js;

mod logging;
mod spawn;
mod creeps;
mod cleanup;

use spawn::run_spawn;
use creeps::run_creep;
use cleanup::run_cleanup;

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
    info!(">>> loop starting! CPU: {}", screeps::game::cpu::get_used());

    // Spawn some units (maybe)
    for spawn in screeps::game::spawns::values() {
        run_spawn(spawn);
    }

    for creep in screeps::game::creeps::values() {
        run_creep(creep);
    }

    run_cleanup();

    info!("<<< done! cpu: {}", screeps::game::cpu::get_used())
}
