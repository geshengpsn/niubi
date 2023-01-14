mod basics;
/**
 * # niubi
 *  A rust lib of freeform curves and surfaces
 */
mod basis_function;
pub mod curve;
pub mod surface;

#[cfg(feature = "na")]
pub mod na;
