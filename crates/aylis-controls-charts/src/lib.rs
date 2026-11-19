//! # aylis-controls-charts
//!
//! Gráficos ([`Chart`]), BI ([`bi`]), o trader ([`trader`]), o grafo de nós ([`NodeGraph`]) e o
//! mapa ([`MapView`]). Os apps usam pelo `aylis-controls`, que registra as classes.

mod chart;
mod graph;
mod map;
mod records;
pub mod bi;
pub mod trader;

pub use chart::{CHART_SERIES, Chart, ChartKind, LegendPosition, PointStyle, TitleAlign, ValueFormat, Series, SeriesFill, SeriesKind, parse_time};
pub use graph::{GraphCable, GraphNode, GraphSelection, NodeGraph};
pub use map::{MAP_MARKER, MAP_POINT, MapMarker, MapPoint, MapView, project, unproject};

pub(crate) use aylis_core::leaf_control;
