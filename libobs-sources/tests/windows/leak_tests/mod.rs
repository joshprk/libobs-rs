//! Six stages:
//! 1. Startup OBS
//! 2. Do above + Create basic output
//! 3. Do above + Create output with encoders
//! 4. Do above + Create scene
//! 5. Do above + Create source and add to scene
//! 6. Do above + Start recording and stop after some time

mod output;
mod output_with_encoders;
mod scene;
mod source;
mod startup;
