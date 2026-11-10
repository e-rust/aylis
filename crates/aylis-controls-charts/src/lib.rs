//! # aylis-controls-charts
//!
//! Gráficos ([`Chart`]), BI ([`bi`]), o trader ([`trader`]), o grafo de nós ([`NodeGraph`]) e o
//! mapa ([`MapView`]). Os apps usam pelo `aylis-controls`, que registra as classes.

mod chart;
mod graph;
mod map;
pub mod bi;
pub mod trader;

pub use chart::{Chart, ChartKind, PointStyle, Series, SeriesFill, SeriesKind, parse_time};
pub use graph::{GraphCable, GraphNode, GraphSelection, NodeGraph};
pub use map::{MapView, project, unproject};

pub(crate) use aylis_core::leaf_control;
