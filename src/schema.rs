// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vegalite {
    /// URL to [JSON schema](http://json-schema.org/) for a Vega-Lite specification. Unless you
    /// have a reason to change this, use `https://vega.github.io/schema/vega-lite/v3.json`.
    /// Setting the `$schema` property allows automatic validation and autocomplete in editors
    /// that support JSON schema.
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Sets how the visualization size should be determined. If a string, should be one of
    /// `"pad"`, `"fit"` or `"none"`.
    /// Object values can additionally specify parameters for content sizing and automatic
    /// resizing.
    /// `"fit"` is only supported for single and layered views that don't use `rangeStep`.
    ///
    /// __Default value__: `pad`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosize: Option<Autosize>,
    /// CSS color property to use as the background of visualization.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Vega-Lite configuration object.  This property can only be defined at the top-level of a
    /// specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// An object describing the data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    /// A global data store for named datasets. This is a mapping from names to inline datasets.
    /// This can be an array of objects or primitive values or a string. Arrays of primitive
    /// values are ingested as objects with a `data` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<HashMap<String, InlineDatasetValue>>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A key-value mapping between encoding channels and definition of fields.
    ///
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// __Default value:__
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its y-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the height will be
    /// the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For y-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the height is [determined by the range step, paddings, and the
    /// cardinality of the field mapped to
    /// y-channel](https://vega.github.io/vega-lite/docs/scale.html#band). Otherwise, if the
    /// `rangeStep` is `null`, the height will be the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `y` channel, the `height` will be the value of `rangeStep`.
    ///
    /// __Note__: For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<AnyMark>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The default visualization padding, in pixels, from the edge of the visualization canvas
    /// to the data rectangle.  If a number, specifies padding for all sides.
    /// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
    /// "bottom": 5}` to specify padding for each side of the visualization.
    ///
    /// __Default value__: `5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    ///
    /// An object defining properties of the geographic projection shared by underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
    /// Optional metadata that will be passed to Vega.
    /// This object is completely ignored by Vega and Vega-Lite and can be used for custom
    /// metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usermeta: Option<HashMap<String, Option<serde_json::Value>>>,
    /// The width of a visualization.
    ///
    /// __Default value:__ This will be determined by the following rules:
    ///
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its x-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the width will be
    /// the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For x-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the width is [determined by the range step, paddings, and the cardinality
    /// of the field mapped to
    /// x-channel](https://vega.github.io/vega-lite/docs/scale.html#band).   Otherwise, if the
    /// `rangeStep` is `null`, the width will be the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `x` channel, the `width` will be the value of
    /// [`config.scale.textXRangeStep`](https://vega.github.io/vega-lite/docs/size.html#default-width-and-height)
    /// for `text` mark and the value of `rangeStep` for other marks.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignUnion>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,
    /// An object that describes mappings between `row` and `column` channels and their field
    /// definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<FacetMapping>,
    /// Scale, axis, and legend resolutions for facets.
    ///
    /// Scale, axis, and legend resolutions for layers.
    ///
    /// Scale and legend resolutions for repeated charts.
    ///
    /// Scale, axis, and legend resolutions for vertically concatenated charts.
    ///
    /// Scale, axis, and legend resolutions for horizontally concatenated charts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<Resolve>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: `10`
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SpecClass>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<Vec<LayerSpec>>,
    /// An object that describes what fields should be repeated into views that are laid out as a
    /// `row` or `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,
    /// A list of views that should be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views that should be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hconcat: Option<Vec<Spec>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowColVgLayoutAlign {
    pub column: Option<VgLayoutAlign>,
    pub row: Option<VgLayoutAlign>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutoSizeParams {
    /// Determines how size calculation should be performed, one of `"content"` or `"padding"`.
    /// The default setting (`"content"`) interprets the width and height settings as the data
    /// rectangle (plotting) dimensions, to which padding is then added. In contrast, the
    /// `"padding"` setting includes the padding within the view size calculations, such that the
    /// width and height settings indicate the **total** intended size of the view.
    ///
    /// __Default value__: `"content"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Contains>,
    /// A boolean flag indicating if autosize layout should be re-calculated on every view
    /// update.
    ///
    /// __Default value__: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize: Option<bool>,
    /// The sizing format type. One of `"pad"`, `"fit"` or `"none"`. See the [autosize
    /// type](https://vega.github.io/vega-lite/docs/size.html#autosize) documentation for
    /// descriptions of each.
    ///
    /// __Default value__: `"pad"`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size_params_type: Option<AutosizeType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowColBoolean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<bool>,
}

/// Vega-Lite configuration object.  This property can only be defined at the top-level of a
/// specification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Area-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaConfig>,
    /// Sets how the visualization size should be determined. If a string, should be one of
    /// `"pad"`, `"fit"` or `"none"`.
    /// Object values can additionally specify parameters for content sizing and automatic
    /// resizing.
    /// `"fit"` is only supported for single and layered views that don't use `rangeStep`.
    ///
    /// __Default value__: `pad`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosize: Option<Autosize>,
    /// Axis configuration, which determines default properties for all `x` and `y`
    /// [axes](https://vega.github.io/vega-lite/docs/axis.html). For a full list of axis
    /// configuration options, please see the [corresponding section of the axis
    /// documentation](https://vega.github.io/vega-lite/docs/axis.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<AxisConfig>,
    /// Specific axis config for axes with "band" scales.
    #[serde(rename = "axisBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_band: Option<AxisConfig>,
    /// Specific axis config for x-axis along the bottom edge of the chart.
    #[serde(rename = "axisBottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_bottom: Option<AxisConfig>,
    /// Specific axis config for y-axis along the left edge of the chart.
    #[serde(rename = "axisLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_left: Option<AxisConfig>,
    /// Specific axis config for y-axis along the right edge of the chart.
    #[serde(rename = "axisRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_right: Option<AxisConfig>,
    /// Specific axis config for x-axis along the top edge of the chart.
    #[serde(rename = "axisTop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_top: Option<AxisConfig>,
    /// X-axis specific config.
    #[serde(rename = "axisX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_x: Option<AxisConfig>,
    /// Y-axis specific config.
    #[serde(rename = "axisY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_y: Option<AxisConfig>,
    /// CSS color property to use as the background of visualization.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Bar-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<BarConfig>,
    /// Box Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxplot: Option<BoxPlotConfig>,
    /// Circle-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle: Option<MarkConfig>,
    /// Default axis and legend title for count fields.
    ///
    /// __Default value:__ `'Number of Records'`.
    #[serde(rename = "countTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_title: Option<String>,
    /// ErrorBand Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorband: Option<ErrorBandConfig>,
    /// ErrorBar Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errorbar: Option<ErrorBarConfig>,
    /// Defines how Vega-Lite generates title for fields.  There are three possible styles:
    /// - `"verbal"` (Default) - displays function in a verbal style (e.g., "Sum of field",
    /// "Year-month of date", "field (binned)").
    /// - `"function"` - displays function using parentheses and capitalized texts (e.g.,
    /// "SUM(field)", "YEARMONTH(date)", "BIN(field)").
    /// - `"plain"` - displays only the field name without functions (e.g., "field", "date",
    /// "field").
    #[serde(rename = "fieldTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_title: Option<FieldTitle>,
    /// Geoshape-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoshape: Option<MarkConfig>,
    /// Header configuration, which determines default properties for all
    /// [header](https://vega.github.io/vega-lite/docs/header.html). For a full list of header
    /// configuration options, please see the [corresponding section of in the header
    /// documentation](https://vega.github.io/vega-lite/docs/header.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<HeaderConfig>,
    /// Defines how Vega-Lite should handle invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(rename = "invalidValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_values: Option<InvalidValues>,
    /// Legend configuration, which determines default properties for all
    /// [legends](https://vega.github.io/vega-lite/docs/legend.html). For a full list of legend
    /// configuration options, please see the [corresponding section of in the legend
    /// documentation](https://vega.github.io/vega-lite/docs/legend.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendConfig>,
    /// Line-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineConfig>,
    /// Mark Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<MarkConfig>,
    /// D3 Number format for guide labels and text marks. For example "s" for SI units. Use [D3's
    /// number format pattern](https://github.com/d3/d3-format#locale_format).
    #[serde(rename = "numberFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_format: Option<String>,
    /// The default visualization padding, in pixels, from the edge of the visualization canvas
    /// to the data rectangle.  If a number, specifies padding for all sides.
    /// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
    /// "bottom": 5}` to specify padding for each side of the visualization.
    ///
    /// __Default value__: `5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    /// Point-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<MarkConfig>,
    /// Projection configuration, which determines default properties for all
    /// [projections](https://vega.github.io/vega-lite/docs/projection.html). For a full list of
    /// projection configuration options, please see the [corresponding section of the projection
    /// documentation](https://vega.github.io/vega-lite/docs/projection.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// An object hash that defines default range arrays or schemes for using with scales.
    /// For a full list of scale range configuration options, please see the [corresponding
    /// section of the scale
    /// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<HashMap<String, RangeValue>>,
    /// Rect-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<MarkConfig>,
    /// Rule-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<MarkConfig>,
    /// Scale configuration determines default properties for all
    /// [scales](https://vega.github.io/vega-lite/docs/scale.html). For a full list of scale
    /// configuration options, please see the [corresponding section of the scale
    /// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleConfig>,
    /// An object hash for defining default properties for each type of selections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionConfig>,
    /// Square-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square: Option<MarkConfig>,
    /// Default stack offset for stackable mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<StackOffset>,
    /// An object hash that defines key-value mappings to determine default properties for marks
    /// with a given [style](https://vega.github.io/vega-lite/docs/mark.html#mark-def).  The keys
    /// represent styles names; the values have to be valid [mark configuration
    /// objects](https://vega.github.io/vega-lite/docs/mark.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<HashMap<String, VgMarkConfig>>,
    /// Text-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,
    /// Tick-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<TickConfig>,
    /// Default time format for raw time values (without time units) in text marks, legend labels
    /// and header labels.
    ///
    /// __Default value:__ `"%b %d, %Y"`
    /// __Note:__ Axes automatically determine format each label automatically so this config
    /// would not affect axes.
    #[serde(rename = "timeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    /// Title configuration, which determines default properties for all
    /// [titles](https://vega.github.io/vega-lite/docs/title.html). For a full list of title
    /// configuration options, please see the [corresponding section of the title
    /// documentation](https://vega.github.io/vega-lite/docs/title.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VgTitleConfig>,
    /// Trail-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<LineConfig>,
    /// Default properties for [single view
    /// plots](https://vega.github.io/vega-lite/docs/spec.html#single).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ViewConfig>,
}

/// Area-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AreaConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    pub baseline: Option<TextBaseline>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    pub limit: Option<f64>,
    /// A flag for overlaying line on top of area marks, or an object defining the properties of
    /// the overlayed lines.
    ///
    /// - If this value is an empty object (`{}`) or `true`, lines with default properties will
    /// be used.
    ///
    /// - If this value is `false`, no lines would be automatically added to area marks.
    ///
    /// __Default value:__ `false`.
    pub line: Option<Line>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    pub orient: Option<Orient>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    pub tooltip: Option<Tooltip>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlayMarkDef {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    pub baseline: Option<TextBaseline>,
    /// Whether a mark be clipped to the enclosing group’s width and height.
    pub clip: Option<bool>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f64>,
    /// A string or array of strings indicating the name of custom styles to apply to the mark. A
    /// style is a named collection of mark property defaults defined within the [style
    /// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
    /// an array, later styles will override earlier styles. Any [mark
    /// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
    /// defined within the `encoding` will override a style default.
    ///
    /// __Default value:__ The mark's name.  For example, a bar mark will have style `"bar"` by
    /// default.
    /// __Note:__ Any specified style will augment the default style. For example, a bar mark
    /// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
    /// specified style `"foo"` has higher precedence).
    pub style: Option<Style>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    pub tooltip: Option<Tooltip>,
    /// Offset for x2-position.
    #[serde(rename = "x2Offset")]
    pub x2_offset: Option<f64>,
    /// Offset for x-position.
    #[serde(rename = "xOffset")]
    pub x_offset: Option<f64>,
    /// Offset for y2-position.
    #[serde(rename = "y2Offset")]
    pub y2_offset: Option<f64>,
    /// Offset for y-position.
    #[serde(rename = "yOffset")]
    pub y_offset: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TooltipContent {
    pub content: Content,
}

/// Axis configuration, which determines default properties for all `x` and `y`
/// [axes](https://vega.github.io/vega-lite/docs/axis.html). For a full list of axis
/// configuration options, please see the [corresponding section of the axis
/// documentation](https://vega.github.io/vega-lite/docs/axis.html#config).
///
/// Specific axis config for axes with "band" scales.
///
/// Specific axis config for x-axis along the bottom edge of the chart.
///
/// Specific axis config for y-axis along the left edge of the chart.
///
/// Specific axis config for y-axis along the right edge of the chart.
///
/// Specific axis config for x-axis along the top edge of the chart.
///
/// X-axis specific config.
///
/// Y-axis specific config.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxisConfig {
    /// An interpolation fraction indicating where, for `band` scales, axis ticks should be
    /// positioned. A value of `0` places ticks at the left edge of their bands. A value of `0.5`
    /// places ticks in the middle of their bands.
    ///
    /// __Default value:__ `0.5`
    #[serde(rename = "bandPosition")]
    pub band_position: Option<f64>,
    /// A boolean flag indicating if the domain (the axis baseline) should be included as part of
    /// the axis.
    ///
    /// __Default value:__ `true`
    pub domain: Option<bool>,
    /// Color of axis domain line.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "domainColor")]
    pub domain_color: Option<String>,
    /// Opacity of the axis domain line.
    #[serde(rename = "domainOpacity")]
    pub domain_opacity: Option<f64>,
    /// Stroke width of axis domain line
    ///
    /// __Default value:__ `1`
    #[serde(rename = "domainWidth")]
    pub domain_width: Option<f64>,
    /// A boolean flag indicating if grid lines should be included as part of the axis
    ///
    /// __Default value:__ `true` for [continuous
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#continuous) that are not binned;
    /// otherwise, `false`.
    pub grid: Option<bool>,
    /// Color of gridlines.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gridColor")]
    pub grid_color: Option<String>,
    /// The offset (in pixels) into which to begin drawing with the grid dash array.
    #[serde(rename = "gridDash")]
    pub grid_dash: Option<Vec<f64>>,
    /// The stroke opacity of grid (value between [0,1])
    ///
    /// __Default value:__ `1`
    #[serde(rename = "gridOpacity")]
    pub grid_opacity: Option<f64>,
    /// The grid width, in pixels.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "gridWidth")]
    pub grid_width: Option<f64>,
    /// Horizontal text alignment of axis tick labels, overriding the default setting for the
    /// current axis orientation.
    #[serde(rename = "labelAlign")]
    pub label_align: Option<Align>,
    /// The rotation angle of the axis labels.
    ///
    /// __Default value:__ `-90` for nominal and ordinal fields; `0` otherwise.
    #[serde(rename = "labelAngle")]
    pub label_angle: Option<f64>,
    /// Vertical text baseline of axis tick labels, overriding the default setting for the
    /// current axis orientation. Can be `"top"`, `"middle"`, `"bottom"`, or `"alphabetic"`.
    #[serde(rename = "labelBaseline")]
    pub label_baseline: Option<TextBaseline>,
    /// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
    /// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
    /// they exceed the axis range by more than 1 pixel. If this property is a number, it
    /// specifies the pixel tolerance: the maximum amount by which a label bounding box may
    /// exceed the axis range.
    ///
    /// __Default value:__ `false`.
    #[serde(rename = "labelBound")]
    pub label_bound: Option<Label>,
    /// The color of the tick label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    pub label_color: Option<String>,
    /// Indicates if the first and last axis labels should be aligned flush with the scale range.
    /// Flush alignment for a horizontal axis will left-align the first label and right-align the
    /// last label. For vertical axes, bottom and top text baselines are applied instead. If this
    /// property is a number, it also indicates the number of pixels by which to offset the first
    /// and last labels; for example, a value of 2 will flush-align the first and last labels and
    /// also push them 2 pixels outward from the center of the axis. The additional adjustment
    /// can sometimes help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
    #[serde(rename = "labelFlush")]
    pub label_flush: Option<Label>,
    /// Indicates the number of pixels by which to offset flush-adjusted labels. For example, a
    /// value of `2` will push flush-adjusted labels 2 pixels outward from the center of the
    /// axis. Offsets can help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "labelFlushOffset")]
    pub label_flush_offset: Option<f64>,
    /// The font of the tick label.
    #[serde(rename = "labelFont")]
    pub label_font: Option<String>,
    /// The font size of the label, in pixels.
    #[serde(rename = "labelFontSize")]
    pub label_font_size: Option<f64>,
    /// Font weight of axis tick labels.
    #[serde(rename = "labelFontWeight")]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `180`
    #[serde(rename = "labelLimit")]
    pub label_limit: Option<f64>,
    /// The opacity of the labels.
    #[serde(rename = "labelOpacity")]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
    /// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
    /// every other label is used (this works well for standard linear axes). If set to
    /// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
    /// with the last visible label (this often works better for log-scaled axes).
    ///
    /// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
    /// scales; otherwise `false`.
    #[serde(rename = "labelOverlap")]
    pub label_overlap: Option<LabelOverlap>,
    /// The padding, in pixels, between axis and text labels.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "labelPadding")]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the axis.
    ///
    /// __Default value:__ `true`.
    pub labels: Option<bool>,
    /// The maximum extent in pixels that axis ticks and labels should use. This determines a
    /// maximum offset value for axis titles.
    ///
    /// __Default value:__ `undefined`.
    #[serde(rename = "maxExtent")]
    pub max_extent: Option<f64>,
    /// The minimum extent in pixels that axis ticks and labels should use. This determines a
    /// minimum offset value for axis titles.
    ///
    /// __Default value:__ `30` for y-axis; `undefined` for x-axis.
    #[serde(rename = "minExtent")]
    pub min_extent: Option<f64>,
    /// Whether month names and weekday names should be abbreviated.
    ///
    /// __Default value:__  `false`
    #[serde(rename = "shortTimeLabels")]
    pub short_time_labels: Option<bool>,
    /// The color of the axis's tick.
    ///
    /// __Default value:__ `"gray"`
    #[serde(rename = "tickColor")]
    pub tick_color: Option<String>,
    /// Boolean flag indicating if an extra axis tick should be added for the initial position of
    /// the axis. This flag is useful for styling axes for `band` scales such that ticks are
    /// placed on band boundaries rather in the middle of a band. Use in conjunction with
    /// `"bandPostion": 1` and an axis `"padding"` value of `0`.
    #[serde(rename = "tickExtra")]
    pub tick_extra: Option<bool>,
    /// Position offset in pixels to apply to ticks, labels, and gridlines.
    #[serde(rename = "tickOffset")]
    pub tick_offset: Option<f64>,
    /// Opacity of the ticks.
    #[serde(rename = "tickOpacity")]
    pub tick_opacity: Option<f64>,
    /// Boolean flag indicating if pixel position values should be rounded to the nearest
    /// integer.
    ///
    /// __Default value:__ `true`
    #[serde(rename = "tickRound")]
    pub tick_round: Option<bool>,
    /// Boolean value that determines whether the axis should include ticks.
    ///
    /// __Default value:__ `true`
    pub ticks: Option<bool>,
    /// The size in pixels of axis ticks.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "tickSize")]
    pub tick_size: Option<f64>,
    /// The width, in pixels, of ticks.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "tickWidth")]
    pub tick_width: Option<f64>,
    /// Horizontal text alignment of axis titles.
    #[serde(rename = "titleAlign")]
    pub title_align: Option<Align>,
    /// Angle in degrees of axis titles.
    #[serde(rename = "titleAngle")]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for axis titles.
    #[serde(rename = "titleBaseline")]
    pub title_baseline: Option<TextBaseline>,
    /// Color of the title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    pub title_color: Option<String>,
    /// Font of the title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    pub title_font: Option<String>,
    /// Font size of the title.
    #[serde(rename = "titleFontSize")]
    pub title_font_size: Option<f64>,
    /// Font weight of the title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    #[serde(rename = "titleLimit")]
    pub title_limit: Option<f64>,
    /// Opacity of the axis title.
    #[serde(rename = "titleOpacity")]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and axis.
    #[serde(rename = "titlePadding")]
    pub title_padding: Option<f64>,
    /// X-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleX")]
    pub title_x: Option<f64>,
    /// Y-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleY")]
    pub title_y: Option<f64>,
}

/// Bar-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    pub baseline: Option<TextBaseline>,
    /// Offset between bars for binned field.  Ideal value for this is either 0 (Preferred by
    /// statisticians) or 1 (Vega-Lite Default, D3 example style).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "binSpacing")]
    pub bin_spacing: Option<f64>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub color: Option<String>,
    /// The default size of the bars on continuous scales.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "continuousBandSize")]
    pub continuous_band_size: Option<f64>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    pub dir: Option<Dir>,
    /// The default size of the bars with discrete dimensions.  If unspecified, the default size
    /// is  `bandSize-1`,
    /// which provides 1 pixel offset between bars.
    #[serde(rename = "discreteBandSize")]
    pub discrete_band_size: Option<f64>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    pub tooltip: Option<Tooltip>,
}

/// Box Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoxPlotConfig {
    #[serde(rename = "box")]
    pub box_plot_config_box: Option<BooleanOrMarkConfig>,
    /// The extent of the whiskers. Available options include:
    /// - `"min-max"`: min and max are the lower and upper whiskers respectively.
    /// - A number representing multiple of the interquartile range (Q3-Q1).  This number will be
    /// multiplied by the IQR. the product will be added to the third quartile to get the upper
    /// whisker and subtracted from the first quartile to get the lower whisker.
    ///
    /// __Default value:__ `1.5`.
    pub extent: Option<BoxplotExtent>,
    pub median: Option<BooleanOrMarkConfig>,
    pub outliers: Option<BooleanOrMarkConfig>,
    pub rule: Option<BooleanOrMarkConfig>,
    /// Size of the box and median tick of a box plot
    pub size: Option<f64>,
    pub ticks: Option<BooleanOrMarkConfig>,
}

/// Circle-Specific Config
///
/// Geoshape-Specific Config
///
/// Mark Config
///
/// Point-Specific Config
///
/// Rect-Specific Config
///
/// Rule-Specific Config
///
/// Square-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,
}

/// ErrorBand Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorBandConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band: Option<BooleanOrMarkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<BooleanOrMarkConfig>,
    /// The extent of the band. Available options include:
    /// - `"ci"`: Extend the band to the confidence interval of the mean.
    /// - `"stderr"`: The size of band are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the band to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<ErrorbandExtent>,
    /// The line interpolation method for the error band. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The tension parameter for the interpolation type of the error band.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
}

/// ErrorBar Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorBarConfig {
    /// The extent of the rule. Available options include:
    /// - `"ci"`: Extend the rule to the confidence interval of the mean.
    /// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the rule to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<ErrorbandExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<BooleanOrMarkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<BooleanOrMarkConfig>,
}

/// Header configuration, which determines default properties for all
/// [header](https://vega.github.io/vega-lite/docs/header.html). For a full list of header
/// configuration options, please see the [corresponding section of in the header
/// documentation](https://vega.github.io/vega-lite/docs/header.html#config).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeaderConfig {
    /// The rotation angle of the header labels.
    ///
    /// __Default value:__ `0` for column header, `-90` for row header.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_angle: Option<f64>,
    /// The color of the header label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// The font of the header label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font: Option<String>,
    /// The font size of the header label, in pixels.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_size: Option<f64>,
    /// The maximum length of the header label in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_limit: Option<f64>,
    /// The orthogonal distance in pixels by which to displace the title from its position along
    /// the edge of the chart.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_padding: Option<f64>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    ///
    /// __Default value:__ `"middle"` for
    /// [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.
    /// `"start"` for other composite views.
    ///
    /// __Note:__ [For now](https://github.com/vega/vega-lite/issues/2875), `anchor` is only
    /// customizable only for [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.  For other composite
    /// views, `anchor` is always `"start"`.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_anchor: Option<String>,
    /// The rotation angle of the header title.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_baseline: Option<TextBaseline>,
    /// Color of the header title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    /// Font of the header title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font: Option<String>,
    /// Font size of the header title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_size: Option<f64>,
    /// Font weight of the header title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_weight: Option<FontWeight>,
    /// The maximum length of the header title in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_limit: Option<f64>,
    /// The orthogonal distance in pixels by which to displace the title from its position along
    /// the edge of the chart.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_padding: Option<f64>,
}

/// Legend configuration, which determines default properties for all
/// [legends](https://vega.github.io/vega-lite/docs/legend.html). For a full list of legend
/// configuration options, please see the [corresponding section of in the legend
/// documentation](https://vega.github.io/vega-lite/docs/legend.html#config).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegendConfig {
    /// The height in pixels to clip symbol legend entries and limit their size.
    #[serde(rename = "clipHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_height: Option<f64>,
    /// The horizontal padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "columnPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_padding: Option<f64>,
    /// The number of columns in which to arrange symbol legend entries. A value of `0` or lower
    /// indicates a single row with one column per entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<f64>,
    /// Corner radius for the full legend.
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// Background fill color for the full legend.
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<String>,
    /// The default direction (`"horizontal"` or `"vertical"`) for gradient legends.
    ///
    /// __Default value:__ `"vertical"`.
    #[serde(rename = "gradientDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_direction: Option<Orient>,
    /// The maximum allowed length in pixels of color ramp gradient labels.
    #[serde(rename = "gradientLabelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_label_limit: Option<f64>,
    /// Vertical offset in pixels for color ramp gradient labels.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "gradientLabelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_label_offset: Option<f64>,
    /// The length in pixels of the primary axis of a color gradient. This value corresponds to
    /// the height of a vertical gradient or the width of a horizontal gradient.
    ///
    /// __Default value:__ `200`.
    #[serde(rename = "gradientLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_length: Option<f64>,
    /// Opacity of the color gradient.
    #[serde(rename = "gradientOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_opacity: Option<f64>,
    /// The color of the gradient stroke, can be in hex color code or regular color name.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gradientStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_stroke_color: Option<String>,
    /// The width of the gradient stroke, in pixels.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "gradientStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_stroke_width: Option<f64>,
    /// The thickness in pixels of the color gradient. This value corresponds to the width of a
    /// vertical gradient or the height of a horizontal gradient.
    ///
    /// __Default value:__ `16`.
    #[serde(rename = "gradientThickness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_thickness: Option<f64>,
    /// The alignment to apply to symbol legends rows and columns. The supported string values
    /// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
    /// layout documentation](https://vega.github.io/vega/docs/layout).
    ///
    /// __Default value:__ `"each"`.
    #[serde(rename = "gridAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_align: Option<VgLayoutAlign>,
    /// The alignment of the legend label, can be left, center, or right.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_align: Option<Align>,
    /// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    ///
    /// __Default value:__ `"middle"`.
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_baseline: Option<TextBaseline>,
    /// The color of the legend label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// The font of the legend label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font: Option<String>,
    /// The font size of legend label.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_size: Option<f64>,
    /// The font weight of legend label.
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `160`.
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_limit: Option<f64>,
    /// The offset of the legend label.
    #[serde(rename = "labelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_offset: Option<f64>,
    /// Opacity of labels.
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
    /// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
    /// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
    /// performed, removing any label that overlaps with the last visible label (this often works
    /// better for log-scaled axes).
    ///
    /// __Default value:__ `"greedy"` for `log scales otherwise `true`.
    /// *
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_overlap: Option<LabelOverlap>,
    /// Padding in pixels between the legend and legend labels.
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_padding: Option<f64>,
    /// The offset in pixels by which to displace the legend from the data rectangle and axes.
    ///
    /// __Default value:__ `18`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// The orientation of the legend, which determines how the legend is positioned within the
    /// scene. One of "left", "right", "top-left", "top-right", "bottom-left", "bottom-right",
    /// "none".
    ///
    /// __Default value:__ `"right"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<LegendOrient>,
    /// The padding between the border and content of the legend group.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,
    /// The vertical padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "rowPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_padding: Option<f64>,
    /// Whether month names and weekday names should be abbreviated.
    ///
    /// __Default value:__  `false`
    #[serde(rename = "shortTimeLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_time_labels: Option<bool>,
    /// Border stroke color for the full legend.
    #[serde(rename = "strokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    /// Border stroke dash pattern for the full legend.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// Border stroke width for the full legend.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Default fill color for legend symbols. Only applied if there is no `"fill"` scale color
    /// encoding for the legend.
    ///
    /// __Default value:__ `"transparent"`.
    #[serde(rename = "symbolBaseFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_base_fill_color: Option<String>,
    /// Default stroke color for legend symbols. Only applied if there is no `"fill"` scale color
    /// encoding for the legend.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "symbolBaseStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_base_stroke_color: Option<String>,
    /// The default direction (`"horizontal"` or `"vertical"`) for symbol legends.
    ///
    /// __Default value:__ `"vertical"`.
    #[serde(rename = "symbolDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_direction: Option<Orient>,
    /// The color of the legend symbol,
    #[serde(rename = "symbolFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_fill_color: Option<String>,
    /// Horizontal pixel offset for legend symbols.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "symbolOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_offset: Option<f64>,
    /// Opacity of the legend symbols.
    #[serde(rename = "symbolOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_opacity: Option<f64>,
    /// The size of the legend symbol, in pixels.
    ///
    /// __Default value:__ `100`.
    #[serde(rename = "symbolSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_size: Option<f64>,
    /// Stroke color for legend symbols.
    #[serde(rename = "symbolStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_stroke_color: Option<String>,
    /// The width of the symbol's stroke.
    ///
    /// __Default value:__ `1.5`.
    #[serde(rename = "symbolStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_stroke_width: Option<f64>,
    /// Default shape type (such as "circle") for legend symbols.
    /// Can be one of ``"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// * In addition to a set of built-in shapes, custom shapes can be defined using [SVG path
    /// strings](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths).
    /// *
    /// * __Default value:__ `"circle"`.
    /// *
    #[serde(rename = "symbolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,
    /// Horizontal text alignment for legend titles.
    ///
    /// __Default value:__ `"left"`.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_align: Option<Align>,
    /// Vertical text baseline for legend titles.
    ///
    /// __Default value:__ `"top"`.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_baseline: Option<TextBaseline>,
    /// The color of the legend title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    /// The font of the legend title.
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font: Option<String>,
    /// The font size of the legend title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_size: Option<f64>,
    /// The font weight of the legend title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    ///
    /// __Default value:__ `180`.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_limit: Option<f64>,
    /// Opacity of the legend title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and legend.
    ///
    /// __Default value:__ `5`.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_padding: Option<f64>,
}

/// Line-Specific Config
///
/// Trail-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaddingClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,
}

/// Projection configuration, which determines default properties for all
/// [projections](https://vega.github.io/vega-lite/docs/projection.html). For a full list of
/// projection configuration options, please see the [corresponding section of the projection
/// documentation](https://vega.github.io/vega-lite/docs/projection.html#config).
///
/// Any property of Projection can be in config
///
/// An object defining properties of geographic projection, which will be applied to `shape`
/// path for `"geoshape"` marks
/// and to `latitude` and `"longitude"` channels for other marks.
///
/// An object defining properties of the geographic projection shared by underlying layers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projection {
    /// Sets the projection’s center to the specified center, a two-element array of longitude
    /// and latitude in degrees.
    ///
    /// __Default value:__ `[0, 0]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Vec<f64>>,
    /// Sets the projection’s clipping circle radius to the specified angle in degrees. If
    /// `null`, switches to [antimeridian](http://bl.ocks.org/mbostock/3788999) cutting rather
    /// than small-circle clipping.
    #[serde(rename = "clipAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_angle: Option<f64>,
    /// Sets the projection’s viewport clip extent to the specified bounds in pixels. The extent
    /// bounds are specified as an array `[[x0, y0], [x1, y1]]`, where `x0` is the left-side of
    /// the viewport, `y0` is the top, `x1` is the right and `y1` is the bottom. If `null`, no
    /// viewport clipping is performed.
    #[serde(rename = "clipExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_extent: Option<Vec<Vec<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lobes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel: Option<f64>,
    /// Sets the threshold for the projection’s [adaptive
    /// resampling](http://bl.ocks.org/mbostock/3795544) to the specified value in pixels. This
    /// value corresponds to the [Douglas–Peucker
    /// distance](http://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm).
    /// If precision is not specified, returns the projection’s current resampling precision
    /// which defaults to `√0.5 ≅ 0.70710…`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
    /// Sets the projection’s three-axis rotation to the specified angles, which must be a two-
    /// or three-element array of numbers [`lambda`, `phi`, `gamma`] specifying the rotation
    /// angles in degrees about each spherical axis. (These correspond to yaw, pitch and roll.)
    ///
    /// __Default value:__ `[0, 0, 0]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    /// The cartographic projection to use. This value is case-insensitive, for example
    /// `"albers"` and `"Albers"` indicate the same projection type. You can find all valid
    /// projection types [in the
    /// documentation](https://vega.github.io/vega-lite/docs/projection.html#projection-types).
    ///
    /// __Default value:__ `mercator`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<VgProjectionType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VgScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
}

/// Scale configuration determines default properties for all
/// [scales](https://vega.github.io/vega-lite/docs/scale.html). For a full list of scale
/// configuration options, please see the [corresponding section of the scale
/// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaleConfig {
    /// Default inner padding for `x` and `y` band-ordinal scales.
    ///
    /// __Default value:__ `0.1`
    #[serde(rename = "bandPaddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_padding_inner: Option<f64>,
    /// Default outer padding for `x` and `y` band-ordinal scales.
    /// If not specified, by default, band scale's paddingOuter is paddingInner/2.
    #[serde(rename = "bandPaddingOuter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_padding_outer: Option<f64>,
    /// If true, values that exceed the data domain are clamped to either the minimum or maximum
    /// range value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp: Option<bool>,
    /// Default padding for continuous scales.
    ///
    /// __Default:__ `5` for continuous x-scale of a vertical bar and continuous y-scale of a
    /// horizontal bar.; `0` otherwise.
    #[serde(rename = "continuousPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_padding: Option<f64>,
    /// The default max value for mapping quantitative fields to bar's size/bandSize.
    ///
    /// If undefined (default), we will use the scale's `rangeStep` - 1.
    #[serde(rename = "maxBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_band_size: Option<f64>,
    /// The default max value for mapping quantitative fields to text's size/fontSize.
    ///
    /// __Default value:__ `40`
    #[serde(rename = "maxFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_font_size: Option<f64>,
    /// Default max opacity for mapping a field to opacity.
    ///
    /// __Default value:__ `0.8`
    #[serde(rename = "maxOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_opacity: Option<f64>,
    /// Default max value for point size scale.
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<f64>,
    /// Default max strokeWidth for the scale of strokeWidth for rule and line marks and of size
    /// for trail marks.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "maxStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stroke_width: Option<f64>,
    /// The default min value for mapping quantitative fields to bar and tick's size/bandSize
    /// scale with zero=false.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "minBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_band_size: Option<f64>,
    /// The default min value for mapping quantitative fields to tick's size/fontSize scale with
    /// zero=false
    ///
    /// __Default value:__ `8`
    #[serde(rename = "minFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_font_size: Option<f64>,
    /// Default minimum opacity for mapping a field to opacity.
    ///
    /// __Default value:__ `0.3`
    #[serde(rename = "minOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_opacity: Option<f64>,
    /// Default minimum value for point size scale with zero=false.
    ///
    /// __Default value:__ `9`
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<f64>,
    /// Default minimum strokeWidth for the scale of strokeWidth for rule and line marks and of
    /// size for trail marks with zero=false.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "minStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stroke_width: Option<f64>,
    /// Default outer padding for `x` and `y` point-ordinal scales.
    ///
    /// __Default value:__ `0.5`
    #[serde(rename = "pointPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_padding: Option<f64>,
    /// Default range cardinality for
    /// [`quantile`](https://vega.github.io/vega-lite/docs/scale.html#quantile) scale.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "quantileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantile_count: Option<f64>,
    /// Default range cardinality for
    /// [`quantize`](https://vega.github.io/vega-lite/docs/scale.html#quantize) scale.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "quantizeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantize_count: Option<f64>,
    /// Default range step for band and point scales of (1) the `y` channel
    /// and (2) the `x` channel when the mark is not `text`.
    ///
    /// __Default value:__ `21`
    #[serde(rename = "rangeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_step: Option<f64>,
    /// If true, rounds numeric output values to integers.
    /// This can be helpful for snapping to the pixel grid.
    /// (Only available for `x`, `y`, and `size` scales.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<bool>,
    /// Default range step for `x` band and point scales of text marks.
    ///
    /// __Default value:__ `90`
    #[serde(rename = "textXRangeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_x_range_step: Option<f64>,
    /// Use the source data range before aggregation as scale domain instead of aggregated data
    /// for aggregate axis.
    ///
    /// This is equivalent to setting `domain` to `"unaggregate"` for aggregated _quantitative_
    /// fields by default.
    ///
    /// This property only works with aggregate functions that produce values within the raw data
    /// domain (`"mean"`, `"average"`, `"median"`, `"q1"`, `"q3"`, `"min"`, `"max"`). For other
    /// aggregations that produce values outside of the raw data domain (e.g. `"count"`,
    /// `"sum"`), this property is ignored.
    ///
    /// __Default value:__ `false`
    #[serde(rename = "useUnaggregatedDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_unaggregated_domain: Option<bool>,
}

/// An object hash for defining default properties for each type of selections.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectionConfig {
    /// The default definition for an
    /// [`interval`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for an interval selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `interval` to `{"translate": false}` disables the ability to move
    /// interval selections by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<IntervalSelectionConfig>,
    /// The default definition for a
    /// [`multi`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for a multi selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `multi` to `{"toggle": "event.altKey"}` adds additional values to
    /// multi selections when clicking with the alt-key pressed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<MultiSelectionConfig>,
    /// The default definition for a
    /// [`single`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for a single selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `single` to `{"on": "dblclick"}` populates single selections on
    /// double-click by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<SingleSelectionConfig>,
}

/// The default definition for an
/// [`interval`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for an interval selection definition (except `type`) may be specified here.
///
/// For instance, setting `interval` to `{"translate": false}` disables the ability to move
/// interval selections by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntervalSelectionConfig {
    /// Establishes a two-way binding between the interval selection and the scales
    /// used within the same view. This allows a user to interactively pan and
    /// zoom the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<BindEnum>,
    /// By default, all data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<VgLayoutAlign>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encodings: Option<Vec<SingleDefChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// An interval selection also adds a rectangle mark to depict the
    /// extents of the interval. The `mark` property can be used to customize the
    /// appearance of the mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<BrushConfig>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<serde_json::Value>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<SelectionResolution>,
    /// When truthy, allows a user to interactively move an interval selection
    /// back-and-forth. Can be `true`, `false` (to disable panning), or a
    /// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
    /// which must include a start and end event to trigger continuous panning.
    ///
    /// __Default value:__ `true`, which corresponds to
    /// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
    /// clicks and dragging within an interval selection to reposition it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<Translate>,
    /// When truthy, allows a user to interactively resize an interval selection.
    /// Can be `true`, `false` (to disable zooming), or a [Vega event stream
    /// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
    /// only `wheel` events are supported.
    ///
    ///
    /// __Default value:__ `true`, which corresponds to `wheel!`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<Translate>,
}

/// An interval selection also adds a rectangle mark to depict the
/// extents of the interval. The `mark` property can be used to customize the
/// appearance of the mark.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrushConfig {
    /// The fill color of the interval mark.
    ///
    /// __Default value:__ `#333333`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// The fill opacity of the interval mark (a value between 0 and 1).
    ///
    /// __Default value:__ `0.125`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The stroke color of the interval mark.
    ///
    /// __Default value:__ `#ffffff`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// An array of alternating stroke and space lengths,
    /// for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) with which to begin drawing the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke opacity of the interval mark (a value between 0 and 1).
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width of the interval mark.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

/// The default definition for a
/// [`multi`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for a multi selection definition (except `type`) may be specified here.
///
/// For instance, setting `multi` to `{"toggle": "event.altKey"}` adds additional values to
/// multi selections when clicking with the alt-key pressed by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiSelectionConfig {
    /// By default, all data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<VgLayoutAlign>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encodings: Option<Vec<SingleDefChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// See the [nearest transform](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<serde_json::Value>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<SelectionResolution>,
    /// Controls whether data values should be toggled or only ever inserted into
    /// multi selections. Can be `true`, `false` (for insertion only), or a
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/).
    ///
    /// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
    /// data values are toggled when a user interacts with the shift-key pressed).
    ///
    /// See the [toggle transform](https://vega.github.io/vega-lite/docs/toggle.html)
    /// documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle: Option<Translate>,
}

/// The default definition for a
/// [`single`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for a single selection definition (except `type`) may be specified here.
///
/// For instance, setting `single` to `{"on": "dblclick"}` populates single selections on
/// double-click by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleSelectionConfig {
    /// Establish a two-way binding between a single selection and input elements
    /// (also known as dynamic query widgets). A binding takes the form of
    /// Vega's [input element binding definition](https://vega.github.io/vega/docs/signals/#bind)
    /// or can be a mapping between projected field/encodings and binding definitions.
    ///
    /// See the [bind transform](https://vega.github.io/vega-lite/docs/bind.html) documentation
    /// for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<HashMap<String, BindValue>>,
    /// By default, all data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<VgLayoutAlign>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encodings: Option<Vec<SingleDefChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// See the [nearest transform](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<serde_json::Value>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<SelectionResolution>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VgBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VgMarkConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// The pixel area each the point/circle/square.
    /// For example: in the case of circles, the radius is determined in part by the square root
    /// of the size value.
    ///
    /// __Default value:__ `30`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// The tooltip text to show upon mouse hover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<serde_json::Value>,
}

/// Text-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Whether month names and weekday names should be abbreviated.
    #[serde(rename = "shortTimeLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_time_labels: Option<bool>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,
}

/// Tick-Specific Config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickConfig {
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The width of the ticks.
    ///
    /// __Default value:__  2/3 of rangeStep.
    #[serde(rename = "bandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_size: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Thickness of the tick mark.
    ///
    /// __Default value:__  `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,
}

/// Title configuration, which determines default properties for all
/// [titles](https://vega.github.io/vega-lite/docs/title.html). For a full list of title
/// configuration options, please see the [corresponding section of the title
/// documentation](https://vega.github.io/vega-lite/docs/title.html#config).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VgTitleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<TitleAnchor>,
    /// Angle in degrees of title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// Vertical text baseline for title text. One of `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Text color for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Font name for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Font size in pixels for title text.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// Font weight for title text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// The reference frame for the anchor position, one of `"bounds"` (to anchor relative to the
    /// full bounding box) or `"group"` (to anchor relative to the group width or height).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<TitleFrame>,
    /// The maximum allowed length in pixels of legend labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The orthogonal offset in pixels by which to displace the title from its position along
    /// the edge of the chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<LegendOrient>,
}

/// Default properties for [single view
/// plots](https://vega.github.io/vega-lite/docs/spec.html#single).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViewConfig {
    /// Whether the view should be clipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<bool>,
    /// The fill color.
    ///
    /// __Default value:__ (none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ (none)
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The default height of the single plot or each plot in a trellis plot when the
    /// visualization has a continuous (non-ordinal) y-scale with `rangeStep` = `null`.
    ///
    /// __Default value:__ `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The stroke color.
    ///
    /// __Default value:__ (none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    ///
    /// __Default value:__ (none)
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    ///
    /// __Default value:__ (none)
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of miter (default), round or bevel.
    ///
    /// __Default value:__ 'miter'
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The stroke line join method. One of miter (default), round or bevel.
    ///
    /// __Default value:__ 'miter'
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ (none)
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    ///
    /// __Default value:__ (none)
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// The default width of the single plot or each plot in a trellis plot when the
    /// visualization has a continuous (non-ordinal) x-scale or ordinal x-scale with `rangeStep`
    /// = `null`.
    ///
    /// __Default value:__ `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// An object describing the data source
///
/// Secondary data source to lookup in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    /// An object that specifies the format for parsing the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataFormat>,
    /// Provide a placeholder name and bind data at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An URL from which to load the data set. Use the `format.type` property
    /// to ensure the loaded data is correctly parsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The full data set, included inline. This can be an array of objects or primitive values,
    /// an object, or a string.
    /// Arrays of primitive values are ingested as objects with a `data` property. Strings are
    /// parsed according to the specified format type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<DataInlineDataset>,
}

/// An object that specifies the format for parsing the data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataFormat {
    /// If set to `null`, disable type inference based on the spec and only use type inference
    /// based on the data.
    /// Alternatively, a parsing directive object can be provided for explicit data types. Each
    /// property of the object corresponds to a field name, and the value to the desired data
    /// type (one of `"number"`, `"boolean"`, `"date"`, or null (do not parse the field)).
    /// For example, `"parse": {"modified_on": "date"}` parses the `modified_on` field in each
    /// input record a Date value.
    ///
    /// For `"date"`, we parse data based using Javascript's
    /// [`Date.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse).
    /// For Specific date formats can be provided (e.g., `{foo: 'date:"%m%d%Y"'}`), using the
    /// [d3-time-format syntax](https://github.com/d3/d3-time-format#locale_format). UTC date
    /// format parsing is supported similarly (e.g., `{foo: 'utc:"%m%d%Y"'}`). See more about
    /// [UTC time](https://vega.github.io/vega-lite/docs/timeunit.html#utc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<HashMap<String, Option<String>>>,
    /// Type of input data: `"json"`, `"csv"`, `"tsv"`, `"dsv"`.
    /// The default format type is determined by the extension of the file URL.
    /// If no extension is detected, `"json"` will be used by default.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_type: Option<DataFormatType>,
    /// The delimiter between records. The delimiter must be a single character (i.e., a single
    /// 16-bit code unit); so, ASCII delimiters are fine, but emoji delimiters are not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// The JSON property containing the desired data.
    /// This parameter can be used when the loaded JSON file may have surrounding structure or
    /// meta-data.
    /// For example `"property": "values.features"` is equivalent to retrieving
    /// `json.values.features`
    /// from the loaded JSON object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// The name of the TopoJSON object set to convert to a GeoJSON feature collection.
    /// For example, in a map of the world, there may be an object set named `"countries"`.
    /// Using the feature property, we can extract this set and generate a GeoJSON feature object
    /// for each country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// The name of the TopoJSON object set to convert to mesh.
    /// Similar to the `feature` option, `mesh` extracts a named TopoJSON object set.
    /// Unlike the `feature` option, the corresponding geo data is returned as a single, unified
    /// mesh instance, not as individual GeoJSON features.
    /// Extracting a mesh is useful for more efficiently drawing borders or other geographic
    /// elements that you do not need to associate with specific regions such as individual
    /// countries, states or counties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
}

/// A key-value mapping between encoding channels and definition of fields.
///
/// A shared key-value mapping between encoding channels and definition of fields in the
/// underlying layers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Encoding {
    /// Color of the marks – either fill or stroke color based on  the `filled` property of mark
    /// definition.
    /// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
    /// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
    /// `"point"`.
    ///
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_
    /// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
    /// `fill` and `stroke` channels.  If either `fill` or `stroke` channel is specified, `color`
    /// channel will be ignored.
    /// 2) See the scale documentation for more information about customizing [color
    /// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<MarkPropDefWithCondition>,
    /// Horizontal facets for trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<FacetFieldDef>,
    /// Additional levels of detail for grouping data in aggregate views and
    /// in line, trail, and area marks without mapping data to a specific visual channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// Fill color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ When using `fill` channel, `color ` channel will be ignored. To customize both
    /// fill and stroke, please use `fill` and `stroke` channels (not `fill` and `color`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<MarkPropDefWithCondition>,
    /// A URL to load upon mouse click.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<DefWithCondition>,
    /// A data field to use as a unique key for data binding. When a visualization’s data is
    /// updated, the key value will be used to match data elements to existing mark instances.
    /// Use a key channel to enable object constancy for transitions over dynamic data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<FieldDef>,
    /// Latitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<FieldDef>,
    /// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
    /// `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude2: Option<FieldDef>,
    /// Longitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<FieldDef>,
    /// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
    /// and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude2: Option<FieldDef>,
    /// Opacity of the marks – either can be a value or a range.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<MarkPropDefWithCondition>,
    /// Order of the marks.
    /// - For stacked marks, this `order` channel encodes [stack
    /// order](https://vega.github.io/vega-lite/docs/stack.html#order).
    /// - For line and trail marks, this `order` channel encodes order of data points in the
    /// lines. This can be useful for creating [a connected
    /// scatterplot](https://vega.github.io/vega-lite/examples/connected_scatterplot.html).
    /// Setting `order` to `{"value": null}` makes the line marks use the original order in the
    /// data sources.
    /// - Otherwise, this `order` channel encodes layer order of the marks.
    ///
    /// __Note__: In aggregate plots, `order` field should be `aggregate`d to avoid creating
    /// additional aggregation grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Vertical facets for trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<FacetFieldDef>,
    /// For `point` marks the supported values are
    /// `"circle"` (default), `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// or `"triangle-down"`, or else a custom SVG path string.
    /// For `geoshape` marks it should be a field definition of the geojson data
    ///
    /// __Default value:__ If undefined, the default shape depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
    /// property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<MarkPropDefWithCondition>,
    /// Size of the mark.
    /// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
    /// - For `"bar"` and `"tick"` – the bar and tick's size.
    /// - For `"text"` – the text's font size.
    /// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
    /// line with varying size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<MarkPropDefWithCondition>,
    /// Stroke color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ When using `stroke` channel, `color ` channel will be ignored. To customize both
    /// stroke and fill, please use `stroke` and `fill` channels (not `stroke` and `color`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<MarkPropDefWithCondition>,
    /// Text of the `text` mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextClass>,
    /// The tooltip text to show upon mouse hover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<EncodingTooltip>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XClass>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2: Option<X2Class>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<XClass>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2: Option<X2Class>,
}

/// Color of the marks – either fill or stroke color based on  the `filled` property of mark
/// definition.
/// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
/// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
/// `"point"`.
///
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_
/// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
/// `fill` and `stroke` channels.  If either `fill` or `stroke` channel is specified, `color`
/// channel will be ignored.
/// 2) See the scale documentation for more information about customizing [color
/// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
///
/// Fill color of the marks.
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_ When using `fill` channel, `color ` channel will be ignored. To customize both
/// fill and stroke, please use `fill` and `stroke` channels (not `fill` and `color`).
///
/// Opacity of the marks – either can be a value or a range.
///
/// __Default value:__ If undefined, the default opacity depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
///
/// For `point` marks the supported values are
/// `"circle"` (default), `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
/// or `"triangle-down"`, or else a custom SVG path string.
/// For `geoshape` marks it should be a field definition of the geojson data
///
/// __Default value:__ If undefined, the default shape depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
/// property.
///
/// Size of the mark.
/// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
/// - For `"bar"` and `"tick"` – the bar and tick's size.
/// - For `"text"` – the text's font size.
/// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
/// line with varying size)
///
/// Stroke color of the marks.
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_ When using `stroke` channel, `color ` channel will be ignored. To customize both
/// stroke and fill, please use `stroke` and `fill` channels (not `stroke` and `color`).
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
///
/// A ValueDef with Condition<ValueDef | FieldDef>
/// {
/// condition: {field: ...} | {value: ...},
/// value: ...,
/// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkPropDefWithCondition {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// One or more value definition(s) with a selection predicate.
    ///
    /// __Note:__ A field definition's `condition` property can only contain [value
    /// definitions](https://vega.github.io/vega-lite/docs/encoding.html#value-def)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ColorCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// Javascript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order.  For discrete time field, values in the sort array can be [date-time
    /// definition objects](types#datetime). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_prop_def_with_condition_type: Option<Type>,
    /// A constant value in visual domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// Binning properties or boolean flag for determining whether to bin data or not.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinParams {
    /// A value in the binned domain at which to anchor the bins, shifting the bin boundaries if
    /// necessary to ensure that a boundary aligns with the anchor value.
    ///
    /// __Default Value:__ the minimum bin extent value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<f64>,
    /// The number base to use for automatic bin determination (default is base 10).
    ///
    /// __Default value:__ `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,
    /// Scale factors indicating allowable subdivisions. The default value is [5, 2], which
    /// indicates that for base 10 numbers (the default base), the method may consider dividing
    /// bin sizes by 5 and/or 2. For example, for an initial step size of 10, the method can
    /// check if bin sizes of 2 (= 10/5), 5 (= 10/2), or 1 (= 10/(5*2)) might also satisfy the
    /// given constraints.
    ///
    /// __Default value:__ `[5, 2]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divide: Option<Vec<f64>>,
    /// A two-element (`[min, max]`) array indicating the range of desired bin values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<Vec<f64>>,
    /// Maximum number of bins.
    ///
    /// __Default value:__ `6` for `row`, `column` and `shape` channels; `10` for other channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbins: Option<f64>,
    /// A minimum allowable step size (particularly useful for integer values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minstep: Option<f64>,
    /// If true (the default), attempts to make the bin boundaries use human-friendly boundaries,
    /// such as multiples of ten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nice: Option<bool>,
    /// An exact step size to use between bins.
    ///
    /// __Note:__ If provided, options such as maxbins will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    /// An array of allowable step sizes to choose from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalValueDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<LogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionOperand>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Selection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<SelectionOperand>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<SelectionOperand>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<SelectionOperand>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Predicate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<LogicalOperandPredicate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<LogicalOperandPredicate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<LogicalOperandPredicate>>,
    /// The value that the field should be equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal: Option<EqualUnion>,
    /// Field to be filtered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Time unit for the field to be filtered.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// An array of inclusive minimum and maximum values
    /// for a field value of a data item to be included in the filtered data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<Option<PurpleRange>>>,
    /// A set of values that the `field`'s value should be a member of,
    /// for a data item included in the filtered data.
    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<SortElement>>,
    /// The value that the field should be less than.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<Lt>,
    /// The value that the field should be greater than.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<Lt>,
    /// The value that the field should be less than or equals to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<Lt>,
    /// The value that the field should be greater than or equals to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<Lt>,
    /// If set to true the field's value has to be valid, meaning both not `null` and not
    /// [`NaN`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NaN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    /// Filter using a selection name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<Box<SelectionOperand>>,
}

/// Object for defining datetime in Vega-Lite Filter.
/// If both month and quarter are provided, month has higher precedence.
/// `day` cannot be combined with other date.
/// We accept string for month and day names.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DateTime {
    /// Integer value representing the date from 1-31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// Value representing the day of a week.  This can be one of: (1) integer value -- `1`
    /// represents Monday; (2) case-insensitive day name (e.g., `"Monday"`);  (3)
    /// case-insensitive, 3-character short day name (e.g., `"Mon"`).   <br/> **Warning:** A
    /// DateTime definition object with `day`** should not be combined with `year`, `quarter`,
    /// `month`, or `date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Day>,
    /// Integer value representing the hour of a day from 0-23.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    /// Integer value representing the millisecond segment of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milliseconds: Option<f64>,
    /// Integer value representing the minute segment of time from 0-59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<f64>,
    /// One of: (1) integer value representing the month from `1`-`12`. `1` represents January;
    /// (2) case-insensitive month name (e.g., `"January"`);  (3) case-insensitive, 3-character
    /// short month name (e.g., `"Jan"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    /// Integer value representing the quarter of the year (from 1-4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarter: Option<f64>,
    /// Integer value representing the second segment (0-59) of a time value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
    /// A boolean flag indicating if date time is in utc time. If false, the date time is in
    /// local time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc: Option<bool>,
    /// Integer value representing the year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalPredicateMarkPropFieldDefClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<LogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionOperand>,
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// Javascript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order.  For discrete time field, values in the sort array can be [date-time
    /// definition objects](types#datetime). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_def_type: Option<Type>,
}

/// Reference to a repeated value.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepeatRef {
    pub repeat: SingleDefChannel,
}

/// Properties of a legend or boolean flag for determining whether to show it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Legend {
    /// The height in pixels to clip symbol legend entries and limit their size.
    #[serde(rename = "clipHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_height: Option<f64>,
    /// The horizontal padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "columnPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_padding: Option<f64>,
    /// The number of columns in which to arrange symbol legend entries. A value of `0` or lower
    /// indicates a single row with one column per entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<f64>,
    /// Corner radius for the full legend.
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The direction of the legend, one of `"vertical"` (default) or `"horizontal"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Orient>,
    /// Background fill color for the full legend.
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<String>,
    /// The formatting pattern for labels. This is D3's [number format
    /// pattern](https://github.com/d3/d3-format#locale_format) for quantitative fields and D3's
    /// [time format pattern](https://github.com/d3/d3-time-format#locale_format) for time
    /// field.
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more information.
    ///
    /// __Default value:__  derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// quantitative fields and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// temporal fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The length in pixels of the primary axis of a color gradient. This value corresponds to
    /// the height of a vertical gradient or the width of a horizontal gradient.
    ///
    /// __Default value:__ `200`.
    #[serde(rename = "gradientLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_length: Option<f64>,
    /// Opacity of the color gradient.
    #[serde(rename = "gradientOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_opacity: Option<f64>,
    /// The color of the gradient stroke, can be in hex color code or regular color name.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gradientStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_stroke_color: Option<String>,
    /// The width of the gradient stroke, in pixels.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "gradientStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_stroke_width: Option<f64>,
    /// The thickness in pixels of the color gradient. This value corresponds to the width of a
    /// vertical gradient or the height of a horizontal gradient.
    ///
    /// __Default value:__ `16`.
    #[serde(rename = "gradientThickness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_thickness: Option<f64>,
    /// The alignment to apply to symbol legends rows and columns. The supported string values
    /// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
    /// layout documentation](https://vega.github.io/vega/docs/layout).
    ///
    /// __Default value:__ `"each"`.
    #[serde(rename = "gridAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_align: Option<VgLayoutAlign>,
    /// The alignment of the legend label, can be left, center, or right.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_align: Option<Align>,
    /// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    ///
    /// __Default value:__ `"middle"`.
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_baseline: Option<TextBaseline>,
    /// The color of the legend label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// The font of the legend label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font: Option<String>,
    /// The font size of legend label.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_size: Option<f64>,
    /// The font weight of legend label.
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `160`.
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_limit: Option<f64>,
    /// The offset of the legend label.
    #[serde(rename = "labelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_offset: Option<f64>,
    /// Opacity of labels.
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
    /// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
    /// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
    /// performed, removing any label that overlaps with the last visible label (this often works
    /// better for log-scaled axes).
    ///
    /// __Default value:__ `true`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_overlap: Option<LabelOverlap>,
    /// Padding in pixels between the legend and legend labels.
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_padding: Option<f64>,
    /// The offset in pixels by which to displace the legend from the data rectangle and axes.
    ///
    /// __Default value:__ `18`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// The orientation of the legend, which determines how the legend is positioned within the
    /// scene. One of "left", "right", "top-left", "top-right", "bottom-left", "bottom-right",
    /// "none".
    ///
    /// __Default value:__ `"right"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<LegendOrient>,
    /// The padding between the border and content of the legend group.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,
    /// The vertical padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "rowPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_padding: Option<f64>,
    /// Border stroke color for the full legend.
    #[serde(rename = "strokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    /// Border stroke width for the full legend.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// The color of the legend symbol,
    #[serde(rename = "symbolFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_fill_color: Option<String>,
    /// Horizontal pixel offset for legend symbols.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "symbolOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_offset: Option<f64>,
    /// Opacity of the legend symbols.
    #[serde(rename = "symbolOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_opacity: Option<f64>,
    /// The size of the legend symbol, in pixels.
    ///
    /// __Default value:__ `100`.
    #[serde(rename = "symbolSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_size: Option<f64>,
    /// Stroke color for legend symbols.
    #[serde(rename = "symbolStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_stroke_color: Option<String>,
    /// The width of the symbol's stroke.
    ///
    /// __Default value:__ `1.5`.
    #[serde(rename = "symbolStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_stroke_width: Option<f64>,
    /// Default shape type (such as "circle") for legend symbols.
    /// Can be one of ``"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// * In addition to a set of built-in shapes, custom shapes can be defined using [SVG path
    /// strings](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths).
    /// *
    /// * __Default value:__ `"circle"`.
    /// *
    #[serde(rename = "symbolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,
    /// The desired number of tick values for quantitative legends.
    #[serde(rename = "tickCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_count: Option<f64>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Horizontal text alignment for legend titles.
    ///
    /// __Default value:__ `"left"`.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_align: Option<Align>,
    /// Vertical text baseline for legend titles.
    ///
    /// __Default value:__ `"top"`.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_baseline: Option<TextBaseline>,
    /// The color of the legend title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    /// The font of the legend title.
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font: Option<String>,
    /// The font size of the legend title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_size: Option<f64>,
    /// The font weight of the legend title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    ///
    /// __Default value:__ `180`.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_limit: Option<f64>,
    /// Opacity of the legend title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and legend.
    ///
    /// __Default value:__ `5`.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_padding: Option<f64>,
    /// The type of the legend. Use `"symbol"` to create a discrete legend and `"gradient"` for a
    /// continuous color gradient.
    ///
    /// __Default value:__ `"gradient"` for non-binned quantitative fields and temporal fields;
    /// `"symbol"` otherwise.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_type: Option<LegendType>,
    /// Explicitly set the visible legend values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<SortElement>>,
    /// A non-positive integer indicating z-index of the legend.
    /// If zindex is 0, legend should be drawn behind all chart elements.
    /// To put them in front, use zindex = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zindex: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scale {
    /// The logarithm base of the `log` scale (default `10`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,
    /// If `true`, values that exceed the data domain are clamped to either the minimum or
    /// maximum range value
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/config.html#scale-config)'s `clamp` (`true`
    /// by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp: Option<bool>,
    /// Customized domain values.
    ///
    /// For _quantitative_ fields, `domain` can take the form of a two-element array with minimum
    /// and maximum values.  [Piecewise
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#piecewise) can be created by
    /// providing a `domain` with more than two entries.
    /// If the input field is aggregated, `domain` can also be a string value `"unaggregated"`,
    /// indicating that the domain should include the raw data values prior to the aggregation.
    ///
    /// For _temporal_ fields, `domain` can be a two-element array minimum and maximum values, in
    /// the form of either timestamps or the [DateTime definition
    /// objects](https://vega.github.io/vega-lite/docs/types.html#datetime).
    ///
    /// For _ordinal_ and _nominal_ fields, `domain` can be an array that lists valid input
    /// values.
    ///
    /// The `selection` property can be used to [interactively
    /// determine](https://vega.github.io/vega-lite/docs/selection.html#scale-domains) the scale
    /// domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainUnion>,
    /// The exponent of the `pow` scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exponent: Option<f64>,
    /// The interpolation method for range values. By default, a general interpolator for
    /// numbers, dates, strings and colors (in HCL space) is used. For color ranges, this
    /// property allows interpolation in alternative color spaces. Legal values include `rgb`,
    /// `hsl`, `hsl-long`, `lab`, `hcl`, `hcl-long`, `cubehelix` and `cubehelix-long` ('-long'
    /// variants use longer paths in polar coordinate spaces). If object-valued, this property
    /// accepts an object with a string-valued _type_ property and an optional numeric _gamma_
    /// property applicable to rgb and cubehelix interpolators. For more, see the [d3-interpolate
    /// documentation](https://github.com/d3/d3-interpolate).
    ///
    /// * __Default value:__ `hcl`
    ///
    /// __Note:__ Sequential scales do not support `interpolate` as they have a fixed
    /// interpolator.  Since Vega-Lite uses sequential scales for quantitative fields by default,
    /// you have to set the scale `type` to other quantitative scale type such as `"linear"` to
    /// customize `interpolate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<InterpolateUnion>,
    /// Extending the domain so that it starts and ends on nice round values. This method
    /// typically modifies the scale’s domain, and may only extend the bounds to the nearest
    /// round value. Nicing is useful if the domain is computed from data and may be irregular.
    /// For example, for a domain of _[0.201479…, 0.996679…]_, a nice domain might be _[0.2,
    /// 1.0]_.
    ///
    /// For quantitative scales such as linear, `nice` can be either a boolean flag or a number.
    /// If `nice` is a number, it will represent a desired tick count. This allows greater
    /// control over the step size used to extend the bounds, guaranteeing that the returned
    /// ticks will exactly cover the domain.
    ///
    /// For temporal fields with time and utc scales, the `nice` value can be a string indicating
    /// the desired time interval. Legal values are `"millisecond"`, `"second"`, `"minute"`,
    /// `"hour"`, `"day"`, `"week"`, `"month"`, and `"year"`. Alternatively, `time` and `utc`
    /// scales can accept an object-valued interval specifier of the form `{"interval": "month",
    /// "step": 3}`, which includes a desired number of interval steps. Here, the domain would
    /// snap to quarter (Jan, Apr, Jul, Oct) boundaries.
    ///
    /// __Default value:__ `true` for unbinned _quantitative_ fields; `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nice: Option<NiceUnion>,
    /// For _[continuous](https://vega.github.io/vega-lite/docs/scale.html#continuous)_ scales,
    /// expands the scale domain to accommodate the specified number of pixels on each of the
    /// scale range. The scale range must represent pixels for this parameter to function as
    /// intended. Padding adjustment is performed prior to all other adjustments, including the
    /// effects of the zero, nice, domainMin, and domainMax properties.
    ///
    /// For _[band](https://vega.github.io/vega-lite/docs/scale.html#band)_ scales, shortcut for
    /// setting `paddingInner` and `paddingOuter` to the same value.
    ///
    /// For _[point](https://vega.github.io/vega-lite/docs/scale.html#point)_ scales, alias for
    /// `paddingOuter`.
    ///
    /// __Default value:__ For _continuous_ scales, derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `continuousPadding`.
    /// For _band and point_ scales, see `paddingInner` and `paddingOuter`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,
    /// The inner padding (spacing) within each band step of band scales, as a fraction of the
    /// step size. This value must lie in the range [0,1].
    ///
    /// For point scale, this property is invalid as point scales do not have internal band
    /// widths (only step sizes between bands).
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `bandPaddingInner`.
    #[serde(rename = "paddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_inner: Option<f64>,
    /// The outer padding (spacing) at the ends of the range of band and point scales,
    /// as a fraction of the step size. This value must lie in the range [0,1].
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `bandPaddingOuter` for
    /// band scales and `pointPadding` for point scales.
    #[serde(rename = "paddingOuter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_outer: Option<f64>,
    /// The range of the scale. One of:
    ///
    /// - A string indicating a [pre-defined named scale
    /// range](https://vega.github.io/vega-lite/docs/scale.html#range-config) (e.g., example,
    /// `"symbol"`, or `"diverging"`).
    ///
    /// - For [continuous scales](https://vega.github.io/vega-lite/docs/scale.html#continuous),
    /// two-element array indicating  minimum and maximum values, or an array with more than two
    /// entries for specifying a [piecewise
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#piecewise).
    ///
    /// - For [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) and
    /// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales, an
    /// array of desired output values.
    ///
    /// __Notes:__
    ///
    /// 1) For [sequential](https://vega.github.io/vega-lite/docs/scale.html#sequential),
    /// [ordinal](https://vega.github.io/vega-lite/docs/scale.html#ordinal), and discretizing
    /// color scales, you can also specify a color
    /// [`scheme`](https://vega.github.io/vega-lite/docs/scale.html#scheme) instead of `range`.
    ///
    /// 2) Any directly specified `range` for `x` and `y` channels will be ignored. Range can be
    /// customized via the view's corresponding
    /// [size](https://vega.github.io/vega-lite/docs/size.html) (`width` and `height`) or via
    /// [range steps and paddings properties](#range-step) for [band](#band) and [point](#point)
    /// scales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ScaleRange>,
    /// The distance between the starts of adjacent bands or points in
    /// [band](https://vega.github.io/vega-lite/docs/scale.html#band) and
    /// [point](https://vega.github.io/vega-lite/docs/scale.html#point) scales.
    ///
    /// If `rangeStep` is `null` or if the view contains the scale's corresponding
    /// [size](https://vega.github.io/vega-lite/docs/size.html) (`width` for `x` scales and
    /// `height` for `y` scales), `rangeStep` will be automatically determined to fit the size of
    /// the view.
    ///
    /// __Default value:__  derived the [scale
    /// config](https://vega.github.io/vega-lite/docs/config.html#scale-config)'s
    /// `textXRangeStep` (`90` by default) for x-scales of `text` marks and `rangeStep` (`21` by
    /// default) for x-scales of other marks and y-scales.
    ///
    /// __Warning__: If `rangeStep` is `null` and the cardinality of the scale's domain is higher
    /// than `width` or `height`, the rangeStep might become less than one pixel and the mark
    /// might not appear correctly.
    #[serde(rename = "rangeStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_step: Option<f64>,
    /// If `true`, rounds numeric output values to integers. This can be helpful for snapping to
    /// the pixel grid.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<bool>,
    /// A string indicating a color
    /// [scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme) name (e.g.,
    /// `"category10"` or `"viridis"`) or a [scheme parameter
    /// object](https://vega.github.io/vega-lite/docs/scale.html#scheme-params).
    ///
    /// Discrete color schemes may be used with
    /// [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) or
    /// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales.
    /// Continuous color schemes are intended for use with
    /// [sequential](https://vega.github.io/vega-lite/docs/scales.html#sequential) scales.
    ///
    /// For the full list of supported schemes, please refer to the [Vega
    /// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Scheme>,
    /// The type of scale.  Vega-Lite supports the following categories of scale types:
    ///
    /// 1) [**Continuous Scales**](https://vega.github.io/vega-lite/docs/scale.html#continuous)
    /// -- mapping continuous domains to continuous output ranges
    /// ([`"linear"`](https://vega.github.io/vega-lite/docs/scale.html#linear),
    /// [`"pow"`](https://vega.github.io/vega-lite/docs/scale.html#pow),
    /// [`"sqrt"`](https://vega.github.io/vega-lite/docs/scale.html#sqrt),
    /// [`"log"`](https://vega.github.io/vega-lite/docs/scale.html#log),
    /// [`"time"`](https://vega.github.io/vega-lite/docs/scale.html#time),
    /// [`"utc"`](https://vega.github.io/vega-lite/docs/scale.html#utc),
    /// [`"sequential"`](https://vega.github.io/vega-lite/docs/scale.html#sequential)).
    ///
    /// 2) [**Discrete Scales**](https://vega.github.io/vega-lite/docs/scale.html#discrete) --
    /// mapping discrete domains to discrete
    /// ([`"ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#ordinal)) or continuous
    /// ([`"band"`](https://vega.github.io/vega-lite/docs/scale.html#band) and
    /// [`"point"`](https://vega.github.io/vega-lite/docs/scale.html#point)) output ranges.
    ///
    /// 3) [**Discretizing
    /// Scales**](https://vega.github.io/vega-lite/docs/scale.html#discretizing) -- mapping
    /// continuous domains to discrete output ranges
    /// ([`"bin-linear"`](https://vega.github.io/vega-lite/docs/scale.html#bin-linear),
    /// [`"bin-ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#bin-ordinal),
    /// [`"quantile"`](https://vega.github.io/vega-lite/docs/scale.html#quantile),
    /// [`"quantize"`](https://vega.github.io/vega-lite/docs/scale.html#quantize) and
    /// [`"threshold"`](https://vega.github.io/vega-lite/docs/scale.html#threshold).
    ///
    /// __Default value:__ please see the [scale type
    /// table](https://vega.github.io/vega-lite/docs/scale.html#type).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ScaleType>,
    /// If `true`, ensures that a zero baseline value is included in the scale domain.
    ///
    /// __Default value:__ `true` for x and y channels if the quantitative field is not binned
    /// and no custom `domain` is provided; `false` otherwise.
    ///
    /// __Note:__ Log, time, and utc scales do not support `zero`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainClass {
    /// The field name to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The name of a selection.
    pub selection: String,
    /// The encoding channel to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaleInterpolateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    #[serde(rename = "type")]
    pub scale_interpolate_params_type: ScaleInterpolateParamsType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NiceClass {
    pub interval: String,
    pub step: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemeParams {
    /// The number of colors to use in the scheme. This can be useful for scale types such as
    /// `"quantize"`, which use the length of the scale range to determine the number of discrete
    /// bins for the scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    /// For sequential and diverging schemes only, determines the extent of the color range to
    /// use. For example `[0.2, 1]` will rescale the color scheme such that color values in the
    /// range _[0, 0.2)_ are excluded from the scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<Vec<f64>>,
    /// A color scheme name for sequential/ordinal scales (e.g., `"category10"` or `"viridis"`).
    ///
    /// For the full list of supported schemes, please refer to the [Vega
    /// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
    pub name: String,
}

/// A sort definition for sorting a discrete scale in an encoding field definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncodingSortField {
    /// The data [field](https://vega.github.io/vega-lite/docs/field.html) to sort by.
    ///
    /// __Default value:__ If unspecified, defaults to the field specified in the outer data
    /// reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
    /// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
    /// This property is required in cases where the sort field and the data reference field do
    /// not match.
    /// The input data objects will be aggregated, grouped by the encoded data field.
    ///
    /// For a full list of operations, please see the documentation for
    /// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
    pub op: AggregateOp,
    /// The sort order. One of `"ascending"` (default), `"descending"`, or `null` (no not sort).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<VgComparatorOrder>,
}

/// Horizontal facets for trellis plots.
///
/// Vertical facets for trellis plots.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FacetFieldDef {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// An object defining properties of a facet's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// Javascript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order.  For discrete time field, values in the sort array can be [date-time
    /// definition objects](types#datetime). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    pub facet_field_def_type: Type,
}

/// An object defining properties of a facet's header.
///
/// Headers of row / column channels for faceted plots.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// The formatting pattern for labels. This is D3's [number format
    /// pattern](https://github.com/d3/d3-format#locale_format) for quantitative fields and D3's
    /// [time format pattern](https://github.com/d3/d3-time-format#locale_format) for time
    /// field.
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more information.
    ///
    /// __Default value:__  derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// quantitative fields and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// temporal fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rotation angle of the header labels.
    ///
    /// __Default value:__ `0` for column header, `-90` for row header.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_angle: Option<f64>,
    /// The color of the header label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// The font of the header label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font: Option<String>,
    /// The font size of the header label, in pixels.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_size: Option<f64>,
    /// The maximum length of the header label in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_limit: Option<f64>,
    /// The orthogonal distance in pixels by which to displace the title from its position along
    /// the edge of the chart.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_padding: Option<f64>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    ///
    /// __Default value:__ `"middle"` for
    /// [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.
    /// `"start"` for other composite views.
    ///
    /// __Note:__ [For now](https://github.com/vega/vega-lite/issues/2875), `anchor` is only
    /// customizable only for [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.  For other composite
    /// views, `anchor` is always `"start"`.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_anchor: Option<String>,
    /// The rotation angle of the header title.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_baseline: Option<TextBaseline>,
    /// Color of the header title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    /// Font of the header title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font: Option<String>,
    /// Font size of the header title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_size: Option<f64>,
    /// Font weight of the header title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_weight: Option<FontWeight>,
    /// The maximum length of the header title in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_limit: Option<f64>,
    /// The orthogonal distance in pixels by which to displace the title from its position along
    /// the edge of the chart.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_padding: Option<f64>,
}

/// Field Def without scale (and without bin: "binned" support).
///
/// A data field to use as a unique key for data binding. When a visualization’s data is
/// updated, the key value will be used to match data elements to existing mark instances.
/// Use a key channel to enable object constancy for transitions over dynamic data.
///
/// Latitude position of geographically projected marks.
///
/// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
/// `"rule"`.
///
/// Longitude position of geographically projected marks.
///
/// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
/// and  `"rule"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDef {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    pub field_def_type: Type,
}

/// A URL to load upon mouse click.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
///
/// A ValueDef with Condition<ValueDef | FieldDef>
/// {
/// condition: {field: ...} | {value: ...},
/// value: ...,
/// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefWithCondition {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// One or more value definition(s) with a selection predicate.
    ///
    /// __Note:__ A field definition's `condition` property can only contain [value
    /// definitions](https://vega.github.io/vega-lite/docs/encoding.html#value-def)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<HrefCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_with_condition_type: Option<Type>,
    /// A constant value in visual domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalPredicateFieldDefClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<LogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionOperand>,
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_def_type: Option<Type>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderFieldDef {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The sort order. One of `"ascending"` (default) or `"descending"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<VgComparatorOrder>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    pub order_field_def_type: Type,
}

/// Definition object for a constant value of an encoding channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderFieldDefClass {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The sort order. One of `"ascending"` (default) or `"descending"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<VgComparatorOrder>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_type: Option<Type>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// Text of the `text` mark.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
///
/// A ValueDef with Condition<ValueDef | FieldDef>
/// {
/// condition: {field: ...} | {value: ...},
/// value: ...,
/// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextClass {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// One or more value definition(s) with a selection predicate.
    ///
    /// __Note:__ A field definition's `condition` property can only contain [value
    /// definitions](https://vega.github.io/vega-lite/docs/encoding.html#value-def)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TextCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The [formatting pattern](https://vega.github.io/vega-lite/docs/format.html) for a text
    /// field. If not defined, this will be determined automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_def_with_condition_type: Option<Type>,
    /// A constant value in visual domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalPredicateTextFieldDefClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<LogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionOperand>,
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The [formatting pattern](https://vega.github.io/vega-lite/docs/format.html) for a text
    /// field. If not defined, this will be determined automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_def_type: Option<Type>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextFieldDef {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The [formatting pattern](https://vega.github.io/vega-lite/docs/format.html) for a text
    /// field. If not defined, this will be determined automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    pub text_field_def_type: Type,
}

/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
///
/// A ValueDef with Condition<ValueDef | FieldDef>
/// {
/// condition: {field: ...} | {value: ...},
/// value: ...,
/// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextDefWithCondition {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// One or more value definition(s) with a selection predicate.
    ///
    /// __Note:__ A field definition's `condition` property can only contain [value
    /// definitions](https://vega.github.io/vega-lite/docs/encoding.html#value-def)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TextCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// The [formatting pattern](https://vega.github.io/vega-lite/docs/format.html) for a text
    /// field. If not defined, this will be determined automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_def_with_condition_type: Option<Type>,
    /// A constant value in visual domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// X coordinates of the marks, or width of horizontal `"bar"` and `"area"`.
///
/// Y coordinates of the marks, or height of vertical `"bar"` and `"area"`.
///
/// Definition object for a constant value of an encoding channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XClass {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// An object defining properties of axis's gridlines, ticks and labels.
    /// If `null`, the axis for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [axis
    /// properties](https://vega.github.io/vega-lite/docs/axis.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// An object defining the properties of the Impute Operation to be applied.
    /// The field value of the other positional channel is taken as `key` of the `Impute`
    /// Operation.
    /// The field of the `color` channel if specified is used as `groupby` of the `Impute`
    /// Operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impute: Option<ImputeParams>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// Javascript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order.  For discrete time field, values in the sort array can be [date-time
    /// definition objects](types#datetime). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    /// Type of stacking offset if the field should be stacked.
    /// `stack` is only applicable for `x` and `y` channels with continuous domains.
    /// For example, `stack` of `y` can be used to customize stacking for a vertical bar chart.
    ///
    /// `stack` can be one of the following values:
    /// - `"zero"`: stacking with baseline offset at zero value of the scale (for creating
    /// typical stacked [bar](https://vega.github.io/vega-lite/docs/stack.html#bar) and
    /// [area](https://vega.github.io/vega-lite/docs/stack.html#area) chart).
    /// - `"normalize"` - stacking with normalized domain (for creating [normalized stacked bar
    /// and area charts](https://vega.github.io/vega-lite/docs/stack.html#normalized). <br/>
    /// -`"center"` - stacking with center baseline (for
    /// [streamgraph](https://vega.github.io/vega-lite/docs/stack.html#streamgraph)).
    /// - `null` - No-stacking. This will produce layered
    /// [bar](https://vega.github.io/vega-lite/docs/stack.html#layered-bar-chart) and area
    /// chart.
    ///
    /// __Default value:__ `zero` for plots with all of the following conditions are true:
    /// (1) the mark is `bar` or `area`;
    /// (2) the stacked measure channel (x or y) has a linear scale;
    /// (3) At least one of non-position channels mapped to an unaggregated field that is
    /// different from x and y.  Otherwise, `null` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<StackOffset>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_type: Option<Type>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Axis {
    /// An interpolation fraction indicating where, for `band` scales, axis ticks should be
    /// positioned. A value of `0` places ticks at the left edge of their bands. A value of `0.5`
    /// places ticks in the middle of their bands.
    ///
    /// __Default value:__ `0.5`
    #[serde(rename = "bandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_position: Option<f64>,
    /// A boolean flag indicating if the domain (the axis baseline) should be included as part of
    /// the axis.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<bool>,
    /// Color of axis domain line.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "domainColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_color: Option<String>,
    /// Opacity of the axis domain line.
    #[serde(rename = "domainOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_opacity: Option<f64>,
    /// Stroke width of axis domain line
    ///
    /// __Default value:__ `1`
    #[serde(rename = "domainWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_width: Option<f64>,
    /// The formatting pattern for labels. This is D3's [number format
    /// pattern](https://github.com/d3/d3-format#locale_format) for quantitative fields and D3's
    /// [time format pattern](https://github.com/d3/d3-time-format#locale_format) for time
    /// field.
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more information.
    ///
    /// __Default value:__  derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// quantitative fields and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// temporal fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// A boolean flag indicating if grid lines should be included as part of the axis
    ///
    /// __Default value:__ `true` for [continuous
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#continuous) that are not binned;
    /// otherwise, `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<bool>,
    /// Color of gridlines.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gridColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_color: Option<String>,
    /// The offset (in pixels) into which to begin drawing with the grid dash array.
    #[serde(rename = "gridDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_dash: Option<Vec<f64>>,
    /// The stroke opacity of grid (value between [0,1])
    ///
    /// __Default value:__ `1`
    #[serde(rename = "gridOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_opacity: Option<f64>,
    /// The grid width, in pixels.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "gridWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_width: Option<f64>,
    /// Horizontal text alignment of axis tick labels, overriding the default setting for the
    /// current axis orientation.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_align: Option<Align>,
    /// The rotation angle of the axis labels.
    ///
    /// __Default value:__ `-90` for nominal and ordinal fields; `0` otherwise.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_angle: Option<f64>,
    /// Vertical text baseline of axis tick labels, overriding the default setting for the
    /// current axis orientation. Can be `"top"`, `"middle"`, `"bottom"`, or `"alphabetic"`.
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_baseline: Option<TextBaseline>,
    /// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
    /// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
    /// they exceed the axis range by more than 1 pixel. If this property is a number, it
    /// specifies the pixel tolerance: the maximum amount by which a label bounding box may
    /// exceed the axis range.
    ///
    /// __Default value:__ `false`.
    #[serde(rename = "labelBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_bound: Option<Label>,
    /// The color of the tick label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    /// Indicates if the first and last axis labels should be aligned flush with the scale range.
    /// Flush alignment for a horizontal axis will left-align the first label and right-align the
    /// last label. For vertical axes, bottom and top text baselines are applied instead. If this
    /// property is a number, it also indicates the number of pixels by which to offset the first
    /// and last labels; for example, a value of 2 will flush-align the first and last labels and
    /// also push them 2 pixels outward from the center of the axis. The additional adjustment
    /// can sometimes help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
    #[serde(rename = "labelFlush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_flush: Option<Label>,
    /// Indicates the number of pixels by which to offset flush-adjusted labels. For example, a
    /// value of `2` will push flush-adjusted labels 2 pixels outward from the center of the
    /// axis. Offsets can help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "labelFlushOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_flush_offset: Option<f64>,
    /// The font of the tick label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font: Option<String>,
    /// The font size of the label, in pixels.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_size: Option<f64>,
    /// Font weight of axis tick labels.
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `180`
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_limit: Option<f64>,
    /// The opacity of the labels.
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
    /// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
    /// every other label is used (this works well for standard linear axes). If set to
    /// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
    /// with the last visible label (this often works better for log-scaled axes).
    ///
    /// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
    /// scales; otherwise `false`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_overlap: Option<LabelOverlap>,
    /// The padding, in pixels, between axis and text labels.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the axis.
    ///
    /// __Default value:__ `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<bool>,
    /// The maximum extent in pixels that axis ticks and labels should use. This determines a
    /// maximum offset value for axis titles.
    ///
    /// __Default value:__ `undefined`.
    #[serde(rename = "maxExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_extent: Option<f64>,
    /// The minimum extent in pixels that axis ticks and labels should use. This determines a
    /// minimum offset value for axis titles.
    ///
    /// __Default value:__ `30` for y-axis; `undefined` for x-axis.
    #[serde(rename = "minExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_extent: Option<f64>,
    /// The offset, in pixels, by which to displace the axis from the edge of the enclosing group
    /// or data rectangle.
    ///
    /// __Default value:__ derived from the [axis
    /// config](https://vega.github.io/vega-lite/docs/config.html#facet-scale-config)'s `offset`
    /// (`0` by default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// The orientation of the axis. One of `"top"`, `"bottom"`, `"left"` or `"right"`. The
    /// orientation can be used to further specialize the axis type (e.g., a y axis oriented for
    /// the right edge of the chart).
    ///
    /// __Default value:__ `"bottom"` for x-axes and `"left"` for y-axes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<LegendOrient>,
    /// The anchor position of the axis in pixels. For x-axes with top or bottom orientation,
    /// this sets the axis group x coordinate. For y-axes with left or right orientation, this
    /// sets the axis group y coordinate.
    ///
    /// __Default value__: `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,
    /// The color of the axis's tick.
    ///
    /// __Default value:__ `"gray"`
    #[serde(rename = "tickColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_color: Option<String>,
    /// A desired number of ticks, for axes visualizing quantitative scales. The resulting number
    /// may be different so that values are "nice" (multiples of 2, 5, 10) and lie within the
    /// underlying scale's range.
    #[serde(rename = "tickCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_count: Option<f64>,
    /// Boolean flag indicating if an extra axis tick should be added for the initial position of
    /// the axis. This flag is useful for styling axes for `band` scales such that ticks are
    /// placed on band boundaries rather in the middle of a band. Use in conjunction with
    /// `"bandPostion": 1` and an axis `"padding"` value of `0`.
    #[serde(rename = "tickExtra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_extra: Option<bool>,
    /// Position offset in pixels to apply to ticks, labels, and gridlines.
    #[serde(rename = "tickOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_offset: Option<f64>,
    /// Opacity of the ticks.
    #[serde(rename = "tickOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_opacity: Option<f64>,
    /// Boolean flag indicating if pixel position values should be rounded to the nearest
    /// integer.
    ///
    /// __Default value:__ `true`
    #[serde(rename = "tickRound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_round: Option<bool>,
    /// Boolean value that determines whether the axis should include ticks.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<bool>,
    /// The size in pixels of axis ticks.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "tickSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<f64>,
    /// A desired step size for ticks. This property will generate the corresponding `tickCount`
    /// and `values`. It can be useful for [data that are binned before importing into
    /// Vega-Lite](https://vega.github.io/vega-lite/docs/bin.html#binned).
    ///
    /// __Default value__: `undefined`
    #[serde(rename = "tickStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_step: Option<f64>,
    /// The width, in pixels, of ticks.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "tickWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_width: Option<f64>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Horizontal text alignment of axis titles.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_align: Option<Align>,
    /// Angle in degrees of axis titles.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for axis titles.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_baseline: Option<TextBaseline>,
    /// Color of the title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    /// Font of the title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font: Option<String>,
    /// Font size of the title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_size: Option<f64>,
    /// Font weight of the title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_limit: Option<f64>,
    /// Opacity of the axis title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and axis.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_padding: Option<f64>,
    /// X-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_x: Option<f64>,
    /// Y-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_y: Option<f64>,
    /// Explicitly set the visible axis tick values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<SortElement>>,
    /// A non-positive integer indicating z-index of the axis.
    /// If zindex is 0, axes should be drawn behind all chart elements.
    /// To put them in front, use `"zindex = 1"`.
    ///
    /// __Default value:__ `1` (in front of the marks) for actual axis and `0` (behind the marks)
    /// for grids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zindex: Option<f64>,
}

/// An object defining the properties of the Impute Operation to be applied.
/// The field value of the other positional channel is taken as `key` of the `Impute`
/// Operation.
/// The field of the `color` channel if specified is used as `groupby` of the `Impute`
/// Operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImputeParams {
    /// A frame specification as a two-element array used to control the window over which the
    /// specified method is applied. The array entries should either be a number indicating the
    /// offset from the current data object, or null to indicate unbounded rows preceding or
    /// following the current data object.  For example, the value `[-5, 5]` indicates that the
    /// window should include five objects preceding and five objects following the current
    /// object.
    ///
    /// __Default value:__:  `[null, null]` indicating that the window includes all objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<Vec<Option<f64>>>,
    /// Defines the key values that should be considered for imputation.
    /// An array of key values or an object defining a [number
    /// sequence](https://vega.github.io/vega-lite/docs/impute.html#sequence-def).
    ///
    /// If provided, this will be used in addition to the key values observed within the input
    /// data.  If not provided, the values will be derived from all unique values of the `key`
    /// field. For `impute` in `encoding`, the key field is the x-field if the y-field is
    /// imputed, or vice versa.
    ///
    /// If there is no impute grouping, this property _must_ be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyvals: Option<Keyvals>,
    /// The imputation method to use for the field value of imputed data objects.
    /// One of `value`, `mean`, `median`, `max` or `min`.
    ///
    /// __Default value:__  `"value"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<ImputeMethod>,
    /// The field value to use when the imputation `method` is `"value"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImputeSequence {
    /// The starting value of the sequence.
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// The step value between sequence entries.
    /// __Default value:__ `1` or `-1` if `stop < start`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    /// The ending value(exclusive) of the sequence.
    pub stop: f64,
}

/// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
///
/// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
///
/// Field Def without scale (and without bin: "binned" support).
///
/// A data field to use as a unique key for data binding. When a visualization’s data is
/// updated, the key value will be used to match data elements to existing mark instances.
/// Use a key channel to enable object constancy for transitions over dynamic data.
///
/// Latitude position of geographically projected marks.
///
/// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
/// `"rule"`.
///
/// Longitude position of geographically projected marks.
///
/// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
/// and  `"rule"`.
///
/// Definition object for a constant value of an encoding channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct X2Class {
    /// Aggregation function for the field
    /// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<AggregateOp>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - To indicate that the data for the `x` (or `y`) channel are already binned, you can set
    /// the `bin` property of the `x` (or `y`) channel to `"binned"` and map the bin-start field
    /// to `x` (or `y`) and the bin-end field to `x2` (or `y2`). The scale and axis will be
    /// formatted similar to binning in Vega-lite.  To adjust the axis ticks based on the bin
    /// step, you can also set the axis's
    /// [`tickStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __Note:__ Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects
    /// (e.g., `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    ///
    /// __Note:__ `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`).  If the field has an aggregate function, the
    /// function is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is
    /// binned or has a time unit applied, the applied function is shown in parentheses (e.g.,
    /// `"Profit (binned)"`, `"Transaction Date (year-month)"`).  Otherwise, the title is simply
    /// the field name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the [`fieldTitle`
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_type: Option<Type>,
    /// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
    /// `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// An object that describes mappings between `row` and `column` channels and their field
/// definitions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FacetMapping {
    /// Horizontal facets for trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<FacetFieldDef>,
    /// Vertical facets for trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<FacetFieldDef>,
}

/// A specification of the view that gets faceted.
///
/// Layer Spec with encoding and projection
///
/// Unit spec that can have a composite mark and row or column channels.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecClass {
    /// An object describing the data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    ///
    /// A key-value mapping between encoding channels and definition of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// __Default value:__
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its y-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the height will be
    /// the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For y-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the height is [determined by the range step, paddings, and the
    /// cardinality of the field mapped to
    /// y-channel](https://vega.github.io/vega-lite/docs/scale.html#band). Otherwise, if the
    /// `rangeStep` is `null`, the height will be the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `y` channel, the `height` will be the value of `rangeStep`.
    ///
    /// __Note__: For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<Vec<LayerSpec>>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An object defining properties of the geographic projection shared by underlying layers.
    ///
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for layers.
    ///
    /// Scale, axis, and legend resolutions for facets.
    ///
    /// Scale and legend resolutions for repeated charts.
    ///
    /// Scale, axis, and legend resolutions for vertically concatenated charts.
    ///
    /// Scale, axis, and legend resolutions for horizontally concatenated charts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<Resolve>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
    /// The width of a visualization.
    ///
    /// __Default value:__ This will be determined by the following rules:
    ///
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its x-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the width will be
    /// the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For x-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the width is [determined by the range step, paddings, and the cardinality
    /// of the field mapped to
    /// x-channel](https://vega.github.io/vega-lite/docs/scale.html#band).   Otherwise, if the
    /// `rangeStep` is `null`, the width will be the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `x` channel, the `width` will be the value of
    /// [`config.scale.textXRangeStep`](https://vega.github.io/vega-lite/docs/size.html#default-width-and-height)
    /// for `text` mark and the value of `rangeStep` for other marks.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<AnyMark>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignUnion>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,
    /// An object that describes mappings between `row` and `column` channels and their field
    /// definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<FacetMapping>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: `10`
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<SpecClass>>,
    /// An object that describes what fields should be repeated into views that are laid out as a
    /// `row` or `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,
    /// A list of views that should be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views that should be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hconcat: Option<Vec<Spec>>,
}

/// Unit spec that can have a composite mark and row or column channels.
///
/// Layer Spec with encoding and projection
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spec {
    /// An object describing the data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A key-value mapping between encoding channels and definition of fields.
    ///
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// __Default value:__
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its y-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the height will be
    /// the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For y-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the height is [determined by the range step, paddings, and the
    /// cardinality of the field mapped to
    /// y-channel](https://vega.github.io/vega-lite/docs/scale.html#band). Otherwise, if the
    /// `rangeStep` is `null`, the height will be the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `y` channel, the `height` will be the value of `rangeStep`.
    ///
    /// __Note__: For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<AnyMark>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    ///
    /// An object defining properties of the geographic projection shared by underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
    /// The width of a visualization.
    ///
    /// __Default value:__ This will be determined by the following rules:
    ///
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its x-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the width will be
    /// the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For x-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the width is [determined by the range step, paddings, and the cardinality
    /// of the field mapped to
    /// x-channel](https://vega.github.io/vega-lite/docs/scale.html#band).   Otherwise, if the
    /// `rangeStep` is `null`, the width will be the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `x` channel, the `width` will be the value of
    /// [`config.scale.textXRangeStep`](https://vega.github.io/vega-lite/docs/size.html#default-width-and-height)
    /// for `text` mark and the value of `rangeStep` for other marks.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<Vec<LayerSpec>>,
    /// Scale, axis, and legend resolutions for layers.
    ///
    /// Scale, axis, and legend resolutions for facets.
    ///
    /// Scale and legend resolutions for repeated charts.
    ///
    /// Scale, axis, and legend resolutions for vertically concatenated charts.
    ///
    /// Scale, axis, and legend resolutions for horizontally concatenated charts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<Resolve>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignUnion>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Center>,
    /// An object that describes mappings between `row` and `column` channels and their field
    /// definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<FacetMapping>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: `10`
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SpecClass>,
    /// An object that describes what fields should be repeated into views that are laid out as a
    /// `row` or `column`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,
    /// A list of views that should be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views that should be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hconcat: Option<Vec<Spec>>,
}

/// Layer Spec with encoding and projection
///
/// Unit spec that can have a composite mark.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerSpec {
    /// An object describing the data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    ///
    /// A key-value mapping between encoding channels and definition of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<LayerEncoding>,
    /// The height of a visualization.
    ///
    /// __Default value:__
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its y-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the height will be
    /// the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For y-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the height is [determined by the range step, paddings, and the
    /// cardinality of the field mapped to
    /// y-channel](https://vega.github.io/vega-lite/docs/scale.html#band). Otherwise, if the
    /// `rangeStep` is `null`, the height will be the value of
    /// [`config.view.height`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `y` channel, the `height` will be the value of `rangeStep`.
    ///
    /// __Note__: For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<Vec<LayerSpec>>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An object defining properties of the geographic projection shared by underlying layers.
    ///
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<Resolve>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Vec<Transform>>,
    /// The width of a visualization.
    ///
    /// __Default value:__ This will be determined by the following rules:
    ///
    /// - If a view's [`autosize`](https://vega.github.io/vega-lite/docs/size.html#autosize) type
    /// is `"fit"` or its x-channel has a [continuous
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#continuous), the width will be
    /// the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - For x-axis with a band or point scale: if
    /// [`rangeStep`](https://vega.github.io/vega-lite/docs/scale.html#band) is a numeric value
    /// or unspecified, the width is [determined by the range step, paddings, and the cardinality
    /// of the field mapped to
    /// x-channel](https://vega.github.io/vega-lite/docs/scale.html#band).   Otherwise, if the
    /// `rangeStep` is `null`, the width will be the value of
    /// [`config.view.width`](https://vega.github.io/vega-lite/docs/spec.html#config).
    /// - If no field is mapped to `x` channel, the `width` will be the value of
    /// [`config.scale.textXRangeStep`](https://vega.github.io/vega-lite/docs/size.html#default-width-and-height)
    /// for `text` mark and the value of `rangeStep` for other marks.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view.
    ///
    /// __See also:__ The documentation for [width and
    /// height](https://vega.github.io/vega-lite/docs/size.html) contains more examples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<AnyMark>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<HashMap<String, SelectionDef>>,
}

/// A shared key-value mapping between encoding channels and definition of fields in the
/// underlying layers.
///
/// A key-value mapping between encoding channels and definition of fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerEncoding {
    /// Color of the marks – either fill or stroke color based on  the `filled` property of mark
    /// definition.
    /// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
    /// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
    /// `"point"`.
    ///
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_
    /// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
    /// `fill` and `stroke` channels.  If either `fill` or `stroke` channel is specified, `color`
    /// channel will be ignored.
    /// 2) See the scale documentation for more information about customizing [color
    /// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<MarkPropDefWithCondition>,
    /// Additional levels of detail for grouping data in aggregate views and
    /// in line, trail, and area marks without mapping data to a specific visual channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Detail>,
    /// Fill color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ When using `fill` channel, `color ` channel will be ignored. To customize both
    /// fill and stroke, please use `fill` and `stroke` channels (not `fill` and `color`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<MarkPropDefWithCondition>,
    /// A URL to load upon mouse click.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<DefWithCondition>,
    /// A data field to use as a unique key for data binding. When a visualization’s data is
    /// updated, the key value will be used to match data elements to existing mark instances.
    /// Use a key channel to enable object constancy for transitions over dynamic data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<FieldDef>,
    /// Latitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<FieldDef>,
    /// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
    /// `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude2: Option<FieldDef>,
    /// Longitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<FieldDef>,
    /// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
    /// and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude2: Option<FieldDef>,
    /// Opacity of the marks – either can be a value or a range.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<MarkPropDefWithCondition>,
    /// Order of the marks.
    /// - For stacked marks, this `order` channel encodes [stack
    /// order](https://vega.github.io/vega-lite/docs/stack.html#order).
    /// - For line and trail marks, this `order` channel encodes order of data points in the
    /// lines. This can be useful for creating [a connected
    /// scatterplot](https://vega.github.io/vega-lite/examples/connected_scatterplot.html).
    /// Setting `order` to `{"value": null}` makes the line marks use the original order in the
    /// data sources.
    /// - Otherwise, this `order` channel encodes layer order of the marks.
    ///
    /// __Note__: In aggregate plots, `order` field should be `aggregate`d to avoid creating
    /// additional aggregation grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// For `point` marks the supported values are
    /// `"circle"` (default), `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// or `"triangle-down"`, or else a custom SVG path string.
    /// For `geoshape` marks it should be a field definition of the geojson data
    ///
    /// __Default value:__ If undefined, the default shape depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
    /// property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<MarkPropDefWithCondition>,
    /// Size of the mark.
    /// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
    /// - For `"bar"` and `"tick"` – the bar and tick's size.
    /// - For `"text"` – the text's font size.
    /// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
    /// line with varying size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<MarkPropDefWithCondition>,
    /// Stroke color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ When using `stroke` channel, `color ` channel will be ignored. To customize both
    /// stroke and fill, please use `stroke` and `fill` channels (not `stroke` and `color`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<MarkPropDefWithCondition>,
    /// Text of the `text` mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextClass>,
    /// The tooltip text to show upon mouse hover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<EncodingTooltip>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<XClass>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2: Option<X2Class>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<XClass>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2: Option<X2Class>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoxPlotDefClass {
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_box: Option<BooleanOrMarkConfig>,
    /// Whether a composite mark be clipped to the enclosing group’s width and height.
    ///
    /// Whether a mark be clipped to the enclosing group’s width and height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<bool>,
    /// Default color.  Note that `fill` and `stroke` have higher precedence than `color` and
    /// will override `color`.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The extent of the whiskers. Available options include:
    /// - `"min-max"`: min and max are the lower and upper whiskers respectively.
    /// - A number representing multiple of the interquartile range (Q3-Q1).  This number will be
    /// multiplied by the IQR. the product will be added to the third quartile to get the upper
    /// whisker and subtracted from the first quartile to get the lower whisker.
    ///
    /// __Default value:__ `1.5`.
    ///
    /// The extent of the rule. Available options include:
    /// - `"ci"`: Extend the rule to the confidence interval of the mean.
    /// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the rule to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    ///
    /// The extent of the band. Available options include:
    /// - `"ci"`: Extend the band to the confidence interval of the mean.
    /// - `"stderr"`: The size of band are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the band to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<BoxPlotDefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median: Option<BooleanOrMarkConfig>,
    /// The opacity (value between [0,1]) of the mark.
    ///
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// Orientation of the box plot.  This is normally automatically determined based on types of
    /// fields on x and y channels. However, an explicit `orient` be specified when the
    /// orientation is ambiguous.
    ///
    /// __Default value:__ `"vertical"`.
    ///
    /// Orientation of the error bar.  This is normally automatically determined, but can be
    /// specified when the orientation is ambiguous and cannot be automatically determined.
    ///
    /// Orientation of the error band. This is normally automatically determined, but can be
    /// specified when the orientation is ambiguous and cannot be automatically determined.
    ///
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<Orient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outliers: Option<BooleanOrMarkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<BooleanOrMarkConfig>,
    /// Size of the box and median tick of a box plot
    ///
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__ `30` for point, circle, square marks; `rangeStep` - 1 for bar marks
    /// with discrete dimensions; `5` for bar marks with continuous dimensions; `11` for text
    /// marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<BooleanOrMarkConfig>,
    /// The mark type. This could a primitive mark type
    /// (one of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
    /// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`)
    /// or a composite mark type (`"boxplot"`, `"errorband"`, `"errorbar"`).
    ///
    /// The mark type.
    /// One of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
    /// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`.
    #[serde(rename = "type")]
    pub def_type: BoxPlot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band: Option<BooleanOrMarkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<BooleanOrMarkConfig>,
    /// The line interpolation method for the error band. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    ///
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolate: Option<Interpolate>,
    /// The tension parameter for the interpolation type of the error band.
    ///
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Offset between bars for binned field.  Ideal value for this is either 0 (Preferred by
    /// statisticians) or 1 (Vega-Lite Default, D3 example style).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "binSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_spacing: Option<f64>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsis: Option<String>,
    /// Default Fill Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `true` for all marks except `point` and `false` for `point`.
    ///
    /// __Applicable for:__ `bar`, `point`, `circle`, `square`, and `area` marks.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// A flag for overlaying line on top of area marks, or an object defining the properties of
    /// the overlayed lines.
    ///
    /// - If this value is an empty object (`{}`) or `true`, lines with default properties will
    /// be used.
    ///
    /// - If this value is `false`, no lines would be automatically added to area marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<Line>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    /// The default symbol shape to use. One of: `"circle"` (default), `"square"`, `"cross"`,
    /// `"diamond"`, `"triangle-up"`, or `"triangle-down"`, or a custom SVG path.
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// Default Stroke Color.  This has higher precedence than `config.color`
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// A string or array of strings indicating the name of custom styles to apply to the mark. A
    /// style is a named collection of mark property defaults defined within the [style
    /// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
    /// an array, later styles will override earlier styles. Any [mark
    /// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
    /// defined within the `encoding` will override a style default.
    ///
    /// __Default value:__ The mark's name.  For example, a bar mark will have style `"bar"` by
    /// default.
    /// __Note:__ Any specified style will augment the default style. For example, a bar mark
    /// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
    /// specified style `"foo"` has higher precedence).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Thickness of the tick mark.
    ///
    /// __Default value:__  `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `{"content": "encoding"}`, then all fields from `encoding` will be
    /// used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,
    /// Offset for x2-position.
    #[serde(rename = "x2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2_offset: Option<f64>,
    /// Offset for x-position.
    #[serde(rename = "xOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_offset: Option<f64>,
    /// Offset for y2-position.
    #[serde(rename = "y2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2_offset: Option<f64>,
    /// Offset for y-position.
    #[serde(rename = "yOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<f64>,
}

/// Scale, axis, and legend resolutions for facets.
///
/// Defines how scales, axes, and legends from different specs should be combined. Resolve is
/// a mapping from `scale`, `axis`, and `legend` to a mapping from channels to resolutions.
///
/// Scale, axis, and legend resolutions for layers.
///
/// Scale and legend resolutions for repeated charts.
///
/// Scale, axis, and legend resolutions for vertically concatenated charts.
///
/// Scale, axis, and legend resolutions for horizontally concatenated charts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resolve {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<AxisResolveMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendResolveMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleResolveMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxisResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<ResolveMode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegendResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<ResolveMode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaleResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<ResolveMode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectionDef {
    /// Establish a two-way binding between a single selection and input elements
    /// (also known as dynamic query widgets). A binding takes the form of
    /// Vega's [input element binding definition](https://vega.github.io/vega/docs/signals/#bind)
    /// or can be a mapping between projected field/encodings and binding definitions.
    ///
    /// See the [bind transform](https://vega.github.io/vega-lite/docs/bind.html) documentation
    /// for more information.
    ///
    /// Establishes a two-way binding between the interval selection and the scales
    /// used within the same view. This allows a user to interactively pan and
    /// zoom the view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<SelectionDefBind>,
    /// By default, all data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<VgLayoutAlign>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encodings: Option<Vec<SingleDefChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// See the [nearest transform](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<serde_json::Value>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve: Option<SelectionResolution>,
    #[serde(rename = "type")]
    pub selection_def_type: SelectionDefType,
    /// Controls whether data values should be toggled or only ever inserted into
    /// multi selections. Can be `true`, `false` (for insertion only), or a
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/).
    ///
    /// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
    /// data values are toggled when a user interacts with the shift-key pressed).
    ///
    /// See the [toggle transform](https://vega.github.io/vega-lite/docs/toggle.html)
    /// documentation for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle: Option<Translate>,
    /// An interval selection also adds a rectangle mark to depict the
    /// extents of the interval. The `mark` property can be used to customize the
    /// appearance of the mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<BrushConfig>,
    /// When truthy, allows a user to interactively move an interval selection
    /// back-and-forth. Can be `true`, `false` (to disable panning), or a
    /// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
    /// which must include a start and end event to trigger continuous panning.
    ///
    /// __Default value:__ `true`, which corresponds to
    /// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
    /// clicks and dragging within an interval selection to reposition it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<Translate>,
    /// When truthy, allows a user to interactively resize an interval selection.
    /// Can be `true`, `false` (to disable zooming), or a [Vega event stream
    /// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
    /// only `wheel` events are supported.
    ///
    ///
    /// __Default value:__ `true`, which corresponds to `wheel!`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<Translate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TitleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    ///
    /// __Default value:__ `"middle"` for
    /// [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.
    /// `"start"` for other composite views.
    ///
    /// __Note:__ [For now](https://github.com/vega/vega-lite/issues/2875), `anchor` is only
    /// customizable only for [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.  For other composite
    /// views, `anchor` is always `"start"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<TitleAnchor>,
    /// Angle in degrees of title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// Vertical text baseline for title text. One of `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<TextBaseline>,
    /// Text color for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Font name for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Font size in pixels for title text.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// Font weight for title text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// The reference frame for the anchor position, one of `"bounds"` (to anchor relative to the
    /// full bounding box) or `"group"` (to anchor relative to the group width or height).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<TitleFrame>,
    /// The maximum allowed length in pixels of legend labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// The orthogonal offset in pixels by which to displace the title from its position along
    /// the edge of the chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<LegendOrient>,
    /// A [mark style property](https://vega.github.io/vega-lite/docs/config.html#style) to apply
    /// to the title text mark.
    ///
    /// __Default value:__ `"group-title"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// The title text.
    pub text: String,
    /// The integer z-index indicating the layering of the title group relative to other axis,
    /// mark and legend groups.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zindex: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform {
    /// The `filter` property must be one of the predicate definitions:
    ///
    /// 1) an [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string,
    /// where `datum` can be used to refer to the current data object
    ///
    /// 2) one of the field predicates:
    /// [`equal`](https://vega.github.io/vega-lite/docs/filter.html#equal-predicate),
    /// [`lt`](https://vega.github.io/vega-lite/docs/filter.html#lt-predicate),
    /// [`lte`](https://vega.github.io/vega-lite/docs/filter.html#lte-predicate),
    /// [`gt`](https://vega.github.io/vega-lite/docs/filter.html#gt-predicate),
    /// [`gte`](https://vega.github.io/vega-lite/docs/filter.html#gte-predicate),
    /// [`range`](https://vega.github.io/vega-lite/docs/filter.html#range-predicate),
    /// or [`oneOf`](https://vega.github.io/vega-lite/docs/filter.html#one-of-predicate).
    ///
    /// 3) a [selection
    /// predicate](https://vega.github.io/vega-lite/docs/filter.html#selection-predicate)
    ///
    /// 4) a logical operand that combines (1), (2), or (3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LogicalOperandPredicate>,
    /// The field for storing the computed formula value.
    ///
    /// The field or fields for storing the computed formula value.
    /// If `from.fields` is specified, the transform will use the same names for `as`.
    /// If `from.fields` is not specified, `as` has to be a string and we put the whole object
    /// into the data under the specified name.
    ///
    /// The output fields at which to write the start and end bin values.
    ///
    /// The output field to write the timeUnit value.
    ///
    /// Output field names. This can be either a string or an array of strings with
    /// two elements denoting the name for the fields for stack start and stack end
    /// respectively.
    /// If a single string(eg."val") is provided, the end field will be "val_end".
    ///
    /// The output field names for extracted array values.
    ///
    /// __Default value:__ The field name of the corresponding array field
    ///
    /// The output field names for the key and value properties produced by the fold transform.
    /// __Default value:__ `["key", "value"]`
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_as: Option<Style>,
    /// A [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string. Use
    /// the variable `datum` to refer to the current data object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate: Option<String>,
    /// The default value to use if lookup fails.
    ///
    /// __Default value:__ `null`
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_default: Option<String>,
    /// Secondary data reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<LookupData>,
    /// Key in primary data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup: Option<String>,
    /// An object indicating bin properties, or simply `true` for using default bin parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<TransformBin>,
    /// The data field to bin.
    ///
    /// The data field to apply time unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The timeUnit.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
    /// A frame specification as a two-element array used to control the window over which the
    /// specified method is applied. The array entries should either be a number indicating the
    /// offset from the current data object, or null to indicate unbounded rows preceding or
    /// following the current data object.  For example, the value `[-5, 5]` indicates that the
    /// window should include five objects preceding and five objects following the current
    /// object.
    ///
    /// __Default value:__:  `[null, null]` indicating that the window includes all objects.
    ///
    /// A frame specification as a two-element array indicating how the sliding window should
    /// proceed. The array entries should either be a number indicating the offset from the
    /// current data object, or null to indicate unbounded rows preceding or following the
    /// current data object. The default value is `[null, 0]`, indicating that the sliding window
    /// includes the current object and all preceding objects. The value `[-5, 5]` indicates that
    /// the window should include five objects preceding and five objects following the current
    /// object. Finally, `[null, null]` indicates that the window frame should always include all
    /// data objects. The only operators affected are the aggregation operations and the
    /// `first_value`, `last_value`, and `nth_value` window operations. The other window
    /// operations are not affected by this.
    ///
    /// __Default value:__:  `[null, 0]` (includes the current object and all preceding objects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<Vec<Option<f64>>>,
    /// An optional array of fields by which to group the values.
    /// Imputation will then be performed on a per-group basis.
    ///
    /// The data fields to group by. If not specified, a single group containing all data objects
    /// will be used.
    ///
    /// The data fields for partitioning the data objects into separate windows. If unspecified,
    /// all data points will be in a single group.
    ///
    /// The data fields to group by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupby: Option<Vec<String>>,
    /// The data field for which the missing values should be imputed.
    pub impute: Option<String>,
    /// A key field that uniquely identifies data objects within a group.
    /// Missing key values (those occurring in the data but not in the current group) will be
    /// imputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Defines the key values that should be considered for imputation.
    /// An array of key values or an object defining a [number
    /// sequence](https://vega.github.io/vega-lite/docs/impute.html#sequence-def).
    ///
    /// If provided, this will be used in addition to the key values observed within the input
    /// data.  If not provided, the values will be derived from all unique values of the `key`
    /// field. For `impute` in `encoding`, the key field is the x-field if the y-field is
    /// imputed, or vice versa.
    ///
    /// If there is no impute grouping, this property _must_ be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyvals: Option<Keyvals>,
    /// The imputation method to use for the field value of imputed data objects.
    /// One of `value`, `mean`, `median`, `max` or `min`.
    ///
    /// __Default value:__  `"value"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<ImputeMethod>,
    /// The field value to use when the imputation `method` is `"value"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Array of objects that define fields to aggregate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<Vec<AggregatedFieldDef>>,
    /// Indicates if the sliding window frame should ignore peer values. (Peer values are those
    /// considered identical by the sort criteria). The default is false, causing the window
    /// frame to expand to include all peer values. If set to true, the window frame will be
    /// defined by offset values only. This setting only affects those operations that depend on
    /// the window frame, namely aggregation operations and the first_value, last_value, and
    /// nth_value window operations.
    ///
    /// __Default value:__ `false`
    #[serde(rename = "ignorePeers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_peers: Option<bool>,
    /// A sort field definition for sorting data objects within a window. If two data objects are
    /// considered equal by the comparator, they are considered “peer” values of equal rank. If
    /// sort is not specified, the order is undefined: data objects are processed in the order
    /// they are observed and none are considered peers (the ignorePeers parameter is ignored and
    /// treated as if set to `true`).
    ///
    /// Field that determines the order of leaves in the stacked charts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortField>>,
    /// The definition of the fields in the window, and what calculations to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<Vec<WindowFieldDef>>,
    /// Mode for stacking marks.
    /// __Default value:__ `"zero"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<StackOffset>,
    /// The field which is stacked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    /// An array of one or more data fields containing arrays to flatten.
    /// If multiple fields are specified, their array values should have a parallel structure,
    /// ideally with the same length.
    /// If the lengths of parallel arrays do not match,
    /// the longest array will be used with `null` values added for missing entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<Vec<String>>,
    /// An array of data fields indicating the properties to fold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fold: Option<Vec<String>>,
    /// The maximum number of data objects to include in the sample.
    ///
    /// __Default value:__ `1000`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AggregatedFieldDef {
    /// The output field names to use for each aggregated field.
    #[serde(rename = "as")]
    pub aggregated_field_def_as: String,
    /// The data field for which to compute aggregate function. This is required for all
    /// aggregation operations except `"count"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The aggregation operations to apply to the fields, such as sum, average or count.
    /// See the [full list of supported aggregation
    /// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
    /// for more information.
    pub op: AggregateOp,
}

/// Secondary data reference.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LookupData {
    /// Secondary data source to lookup in.
    pub data: Data,
    /// Fields in foreign data to lookup.
    /// If not specified, the entire object is queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Key in data to lookup.
    pub key: String,
}

/// A sort definition for transform
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortField {
    /// The name of the field to sort.
    pub field: String,
    /// Whether to sort the field in ascending or descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<VgComparatorOrder>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowFieldDef {
    /// The output name for the window operation.
    #[serde(rename = "as")]
    pub window_field_def_as: String,
    /// The data field for which to compute the aggregate or window function. This can be omitted
    /// for window functions that do not operate over a field such as `count`, `rank`,
    /// `dense_rank`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The window or aggregation operations to apply within a window, including `rank`, `lead`,
    /// `sum`, `average` or `count`. See the list of all supported operations
    /// [here](https://vega.github.io/vega-lite/docs/window.html#ops).
    pub op: Op,
    /// Parameter values for the window functions. Parameter values can be omitted for operations
    /// that do not accept a parameter.
    ///
    /// See the list of all supported operations and their parameters
    /// [here](https://vega.github.io/vega-lite/docs/transforms/window.html).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<f64>,
}

/// An object that describes what fields should be repeated into views that are laid out as a
/// `row` or `column`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repeat {
    /// Horizontal repeated views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<Vec<String>>,
    /// Vertical repeated views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowColNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<f64>,
}

/// The alignment to apply to grid rows and columns.
/// The supported string values are `"all"`, `"each"`, and `"none"`.
///
/// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
/// one after the other.
/// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
/// column may be of variable size.
/// - For `"all"`, subviews will be aligned and each row or column will be sized identically
/// based on the maximum observed size. String values for this property will be applied to
/// both grid rows and columns.
///
/// Alternatively, an object value of the form `{"row": string, "column": string}` can be
/// used to supply different alignments for rows and columns.
///
/// __Default value:__ `"all"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlignUnion {
    Enum(VgLayoutAlign),
    RowColVgLayoutAlign(RowColVgLayoutAlign),
}

/// Sets how the visualization size should be determined. If a string, should be one of
/// `"pad"`, `"fit"` or `"none"`.
/// Object values can additionally specify parameters for content sizing and automatic
/// resizing.
/// `"fit"` is only supported for single and layered views that don't use `rangeStep`.
///
/// __Default value__: `pad`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Autosize {
    AutoSizeParams(AutoSizeParams),
    Enum(AutosizeType),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Center {
    Bool(bool),
    RowColBoolean(RowColBoolean),
}

/// The font weight.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight of axis tick labels.
///
/// Font weight of the title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight of the header title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// The font weight of legend label.
///
/// The font weight of the legend title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight for title text.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FontWeight {
    Double(f64),
    Enum(FontWeightEnum),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Line {
    Bool(bool),
    OverlayMarkDef(OverlayMarkDef),
}

/// A string or array of strings indicating the name of custom styles to apply to the mark. A
/// style is a named collection of mark property defaults defined within the [style
/// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
/// an array, later styles will override earlier styles. Any [mark
/// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
/// defined within the `encoding` will override a style default.
///
/// __Default value:__ The mark's name.  For example, a bar mark will have style `"bar"` by
/// default.
/// __Note:__ Any specified style will augment the default style. For example, a bar mark
/// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
/// specified style `"foo"` has higher precedence).
///
/// A [mark style property](https://vega.github.io/vega-lite/docs/config.html#style) to apply
/// to the title text mark.
///
/// __Default value:__ `"group-title"`.
///
/// The field or fields for storing the computed formula value.
/// If `from.fields` is specified, the transform will use the same names for `as`.
/// If `from.fields` is not specified, `as` has to be a string and we put the whole object
/// into the data under the specified name.
///
/// The output fields at which to write the start and end bin values.
///
/// Output field names. This can be either a string or an array of strings with
/// two elements denoting the name for the fields for stack start and stack end
/// respectively.
/// If a single string(eg."val") is provided, the end field will be "val_end".
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Style {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tooltip {
    String(String),
    TooltipContent(TooltipContent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointUnion {
    Bool(bool),
    Enum(PointEnum),
    OverlayMarkDef(OverlayMarkDef),
}

/// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
/// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
/// they exceed the axis range by more than 1 pixel. If this property is a number, it
/// specifies the pixel tolerance: the maximum amount by which a label bounding box may
/// exceed the axis range.
///
/// __Default value:__ `false`.
///
/// Indicates if the first and last axis labels should be aligned flush with the scale range.
/// Flush alignment for a horizontal axis will left-align the first label and right-align the
/// last label. For vertical axes, bottom and top text baselines are applied instead. If this
/// property is a number, it also indicates the number of pixels by which to offset the first
/// and last labels; for example, a value of 2 will flush-align the first and last labels and
/// also push them 2 pixels outward from the center of the axis. The additional adjustment
/// can sometimes help the labels better visually group with corresponding axis ticks.
///
/// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Label {
    Bool(bool),
    Double(f64),
}

/// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
/// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
/// every other label is used (this works well for standard linear axes). If set to
/// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
/// with the last visible label (this often works better for log-scaled axes).
///
/// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
/// scales; otherwise `false`.
///
/// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
/// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
/// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
/// performed, removing any label that overlaps with the last visible label (this often works
/// better for log-scaled axes).
///
/// __Default value:__ `"greedy"` for `log scales otherwise `true`.
/// *
///
/// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
/// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
/// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
/// performed, removing any label that overlaps with the last visible label (this often works
/// better for log-scaled axes).
///
/// __Default value:__ `true`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LabelOverlap {
    Bool(bool),
    Enum(LabelOverlapEnum),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanOrMarkConfig {
    Bool(bool),
    MarkConfig(MarkConfig),
}

/// The extent of the whiskers. Available options include:
/// - `"min-max"`: min and max are the lower and upper whiskers respectively.
/// - A number representing multiple of the interquartile range (Q3-Q1).  This number will be
/// multiplied by the IQR. the product will be added to the third quartile to get the upper
/// whisker and subtracted from the first quartile to get the lower whisker.
///
/// __Default value:__ `1.5`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoxplotExtent {
    Double(f64),
    Enum(ExtentEnum),
}

/// The default visualization padding, in pixels, from the edge of the visualization canvas
/// to the data rectangle.  If a number, specifies padding for all sides.
/// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
/// "bottom": 5}` to specify padding for each side of the visualization.
///
/// __Default value__: `5`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Padding {
    Double(f64),
    PaddingClass(PaddingClass),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeValue {
    UnionArray(Vec<RangeElement>),
    VgScheme(VgScheme),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeElement {
    Double(f64),
    String(String),
}

/// When truthy, allows a user to interactively move an interval selection
/// back-and-forth. Can be `true`, `false` (to disable panning), or a
/// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
/// which must include a start and end event to trigger continuous panning.
///
/// __Default value:__ `true`, which corresponds to
/// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
/// clicks and dragging within an interval selection to reposition it.
///
/// When truthy, allows a user to interactively resize an interval selection.
/// Can be `true`, `false` (to disable zooming), or a [Vega event stream
/// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
/// only `wheel` events are supported.
///
///
/// __Default value:__ `true`, which corresponds to `wheel!`.
///
/// Controls whether data values should be toggled or only ever inserted into
/// multi selections. Can be `true`, `false` (for insertion only), or a
/// [Vega expression](https://vega.github.io/vega/docs/expressions/).
///
/// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
/// data values are toggled when a user interacts with the shift-key pressed).
///
/// See the [toggle transform](https://vega.github.io/vega-lite/docs/toggle.html)
/// documentation for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Translate {
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BindValue {
    Double(f64),
    String(String),
    StringArray(Vec<String>),
    VgBinding(VgBinding),
}

/// The full data set, included inline. This can be an array of objects or primitive values,
/// an object, or a string.
/// Arrays of primitive values are ingested as objects with a `data` property. Strings are
/// parsed according to the specified format type.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataInlineDataset {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    String(String),
    UnionArray(Vec<InlineDataset>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineDataset {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    Bool(bool),
    Double(f64),
    String(String),
}

/// The full data set, included inline. This can be an array of objects or primitive values,
/// an object, or a string.
/// Arrays of primitive values are ingested as objects with a `data` property. Strings are
/// parsed according to the specified format type.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineDatasetValue {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    String(String),
    UnionArray(Vec<InlineDataset>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    BinParams(BinParams),
    Bool(bool),
    Enum(BinEnum),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorCondition {
    ConditionalPredicateMarkPropFieldDefClass(ConditionalPredicateMarkPropFieldDefClass),
    ConditionalValueDefArray(Vec<ConditionalValueDef>),
}

/// Filter using a selection name.
///
/// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
/// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectionOperand {
    Selection(Selection),
    String(String),
}

/// The `filter` property must be one of the predicate definitions:
///
/// 1) an [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string,
/// where `datum` can be used to refer to the current data object
///
/// 2) one of the field predicates:
/// [`equal`](https://vega.github.io/vega-lite/docs/filter.html#equal-predicate),
/// [`lt`](https://vega.github.io/vega-lite/docs/filter.html#lt-predicate),
/// [`lte`](https://vega.github.io/vega-lite/docs/filter.html#lte-predicate),
/// [`gt`](https://vega.github.io/vega-lite/docs/filter.html#gt-predicate),
/// [`gte`](https://vega.github.io/vega-lite/docs/filter.html#gte-predicate),
/// [`range`](https://vega.github.io/vega-lite/docs/filter.html#range-predicate),
/// or [`oneOf`](https://vega.github.io/vega-lite/docs/filter.html#one-of-predicate).
///
/// 3) a [selection
/// predicate](https://vega.github.io/vega-lite/docs/filter.html#selection-predicate)
///
/// 4) a logical operand that combines (1), (2), or (3).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogicalOperandPredicate {
    Predicate(Predicate),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EqualUnion {
    Bool(bool),
    DateTime(DateTime),
    Double(f64),
    String(String),
}

/// Value representing the day of a week.  This can be one of: (1) integer value -- `1`
/// represents Monday; (2) case-insensitive day name (e.g., `"Monday"`);  (3)
/// case-insensitive, 3-character short day name (e.g., `"Mon"`).   <br/> **Warning:** A
/// DateTime definition object with `day`** should not be combined with `year`, `quarter`,
/// `month`, or `date`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Day {
    Double(f64),
    String(String),
}

/// One of: (1) integer value representing the month from `1`-`12`. `1` represents January;
/// (2) case-insensitive month name (e.g., `"January"`);  (3) case-insensitive, 3-character
/// short month name (e.g., `"Jan"`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Month {
    Double(f64),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Lt {
    DateTime(DateTime),
    Double(f64),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortElement {
    Bool(bool),
    DateTime(DateTime),
    Double(f64),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleRange {
    DateTime(DateTime),
    Double(f64),
}

/// A constant value in visual domain (e.g., `"red"` / "#0099ff" for color, values between
/// `0` to `1` for opacity).
///
/// A constant value in visual domain.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    Double(f64),
    String(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    RepeatRef(RepeatRef),
    String(String),
}

/// Customized domain values.
///
/// For _quantitative_ fields, `domain` can take the form of a two-element array with minimum
/// and maximum values.  [Piecewise
/// scales](https://vega.github.io/vega-lite/docs/scale.html#piecewise) can be created by
/// providing a `domain` with more than two entries.
/// If the input field is aggregated, `domain` can also be a string value `"unaggregated"`,
/// indicating that the domain should include the raw data values prior to the aggregation.
///
/// For _temporal_ fields, `domain` can be a two-element array minimum and maximum values, in
/// the form of either timestamps or the [DateTime definition
/// objects](https://vega.github.io/vega-lite/docs/types.html#datetime).
///
/// For _ordinal_ and _nominal_ fields, `domain` can be an array that lists valid input
/// values.
///
/// The `selection` property can be used to [interactively
/// determine](https://vega.github.io/vega-lite/docs/selection.html#scale-domains) the scale
/// domain.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainUnion {
    DomainClass(DomainClass),
    Enum(Domain),
    UnionArray(Vec<SortElement>),
}

/// The interpolation method for range values. By default, a general interpolator for
/// numbers, dates, strings and colors (in HCL space) is used. For color ranges, this
/// property allows interpolation in alternative color spaces. Legal values include `rgb`,
/// `hsl`, `hsl-long`, `lab`, `hcl`, `hcl-long`, `cubehelix` and `cubehelix-long` ('-long'
/// variants use longer paths in polar coordinate spaces). If object-valued, this property
/// accepts an object with a string-valued _type_ property and an optional numeric _gamma_
/// property applicable to rgb and cubehelix interpolators. For more, see the [d3-interpolate
/// documentation](https://github.com/d3/d3-interpolate).
///
/// * __Default value:__ `hcl`
///
/// __Note:__ Sequential scales do not support `interpolate` as they have a fixed
/// interpolator.  Since Vega-Lite uses sequential scales for quantitative fields by default,
/// you have to set the scale `type` to other quantitative scale type such as `"linear"` to
/// customize `interpolate`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterpolateUnion {
    Enum(ScaleInterpolate),
    ScaleInterpolateParams(ScaleInterpolateParams),
}

/// Extending the domain so that it starts and ends on nice round values. This method
/// typically modifies the scale’s domain, and may only extend the bounds to the nearest
/// round value. Nicing is useful if the domain is computed from data and may be irregular.
/// For example, for a domain of _[0.201479…, 0.996679…]_, a nice domain might be _[0.2,
/// 1.0]_.
///
/// For quantitative scales such as linear, `nice` can be either a boolean flag or a number.
/// If `nice` is a number, it will represent a desired tick count. This allows greater
/// control over the step size used to extend the bounds, guaranteeing that the returned
/// ticks will exactly cover the domain.
///
/// For temporal fields with time and utc scales, the `nice` value can be a string indicating
/// the desired time interval. Legal values are `"millisecond"`, `"second"`, `"minute"`,
/// `"hour"`, `"day"`, `"week"`, `"month"`, and `"year"`. Alternatively, `time` and `utc`
/// scales can accept an object-valued interval specifier of the form `{"interval": "month",
/// "step": 3}`, which includes a desired number of interval steps. Here, the domain would
/// snap to quarter (Jan, Apr, Jul, Oct) boundaries.
///
/// __Default value:__ `true` for unbinned _quantitative_ fields; `false` otherwise.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NiceUnion {
    Bool(bool),
    Double(f64),
    Enum(NiceTime),
    NiceClass(NiceClass),
}

/// The range of the scale. One of:
///
/// - A string indicating a [pre-defined named scale
/// range](https://vega.github.io/vega-lite/docs/scale.html#range-config) (e.g., example,
/// `"symbol"`, or `"diverging"`).
///
/// - For [continuous scales](https://vega.github.io/vega-lite/docs/scale.html#continuous),
/// two-element array indicating  minimum and maximum values, or an array with more than two
/// entries for specifying a [piecewise
/// scale](https://vega.github.io/vega-lite/docs/scale.html#piecewise).
///
/// - For [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) and
/// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales, an
/// array of desired output values.
///
/// __Notes:__
///
/// 1) For [sequential](https://vega.github.io/vega-lite/docs/scale.html#sequential),
/// [ordinal](https://vega.github.io/vega-lite/docs/scale.html#ordinal), and discretizing
/// color scales, you can also specify a color
/// [`scheme`](https://vega.github.io/vega-lite/docs/scale.html#scheme) instead of `range`.
///
/// 2) Any directly specified `range` for `x` and `y` channels will be ignored. Range can be
/// customized via the view's corresponding
/// [size](https://vega.github.io/vega-lite/docs/size.html) (`width` and `height`) or via
/// [range steps and paddings properties](#range-step) for [band](#band) and [point](#point)
/// scales.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleRange {
    String(String),
    UnionArray(Vec<RangeElement>),
}

/// A string indicating a color
/// [scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme) name (e.g.,
/// `"category10"` or `"viridis"`) or a [scheme parameter
/// object](https://vega.github.io/vega-lite/docs/scale.html#scheme-params).
///
/// Discrete color schemes may be used with
/// [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) or
/// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales.
/// Continuous color schemes are intended for use with
/// [sequential](https://vega.github.io/vega-lite/docs/scales.html#sequential) scales.
///
/// For the full list of supported schemes, please refer to the [Vega
/// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Scheme {
    SchemeParams(SchemeParams),
    String(String),
}

/// Sort order for the encoded field.
///
/// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
/// `"descending"`.
///
/// For discrete fields, `sort` can be one of the following:
/// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
/// Javascript.
/// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
/// for sorting by another field.
/// - [An array specifying the field values in preferred
/// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
/// sort order will obey the values in the array, followed by any unspecified values in their
/// original order.  For discrete time field, values in the sort array can be [date-time
/// definition objects](types#datetime). In addition, for time units `"month"` and `"day"`,
/// the values can be the month or day names (case insensitive) or their 3-letter initials
/// (e.g., `"Mon"`, `"Tue"`).
/// - `null` indicating no sort.
///
/// __Default value:__ `"ascending"`
///
/// __Note:__ `null` is not supported for `row` and `column`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sort {
    EncodingSortField(EncodingSortField),
    Enum(VgComparatorOrder),
    UnionArray(Vec<SortElement>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Detail {
    FieldDef(FieldDef),
    FieldDefArray(Vec<FieldDef>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HrefCondition {
    ConditionalPredicateFieldDefClass(ConditionalPredicateFieldDefClass),
    ConditionalValueDefArray(Vec<ConditionalValueDef>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Order {
    OrderFieldDefArray(Vec<OrderFieldDef>),
    OrderFieldDefClass(OrderFieldDefClass),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextCondition {
    ConditionalPredicateTextFieldDefClass(ConditionalPredicateTextFieldDefClass),
    ConditionalValueDefArray(Vec<ConditionalValueDef>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EncodingTooltip {
    TextDefWithCondition(TextDefWithCondition),
    TextFieldDefArray(Vec<TextFieldDef>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keyvals {
    AnythingArray(Vec<Option<serde_json::Value>>),
    ImputeSequence(ImputeSequence),
}

/// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
/// `"line"`,
/// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
/// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyMark {
    BoxPlotDefClass(BoxPlotDefClass),
    Enum(BoxPlot),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoxPlotDefExtent {
    Double(f64),
    Enum(ExtentExtent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectionDefBind {
    Enum(BindEnum),
    UnionMap(HashMap<String, BindValue>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Title {
    String(String),
    TitleParams(TitleParams),
}

/// An object indicating bin properties, or simply `true` for using default bin parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransformBin {
    BinParams(BinParams),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spacing {
    Double(f64),
    RowColNumber(RowColNumber),
}

/// The alignment to apply to symbol legends rows and columns. The supported string values
/// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
/// layout documentation](https://vega.github.io/vega/docs/layout).
///
/// __Default value:__ `"each"`.
///
/// By default, all data values are considered to lie within an empty selection.
/// When set to `none`, empty selections contain no data values.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VgLayoutAlign {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "each")]
    Each,
    #[serde(rename = "none")]
    None,
}

/// The sizing format type. One of `"pad"`, `"fit"` or `"none"`. See the [autosize
/// type](https://vega.github.io/vega-lite/docs/size.html#autosize) documentation for
/// descriptions of each.
///
/// __Default value__: `"pad"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AutosizeType {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pad")]
    Pad,
}

/// Determines how size calculation should be performed, one of `"content"` or `"padding"`.
/// The default setting (`"content"`) interprets the width and height settings as the data
/// rectangle (plotting) dimensions, to which padding is then added. In contrast, the
/// `"padding"` setting includes the padding within the view size calculations, such that the
/// width and height settings indicate the **total** intended size of the view.
///
/// __Default value__: `"content"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Contains {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "padding")]
    Padding,
}

/// The bounds calculation method to use for determining the extent of a sub-plot. One of
/// `full` (the default) or `flush`.
///
/// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
/// be used.
/// - If set to `flush`, only the specified width and height values for the sub-view will be
/// used. The `flush` setting can be useful when attempting to place sub-plots without axes
/// or legends into a uniform grid structure.
///
/// __Default value:__ `"full"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Bounds {
    #[serde(rename = "flush")]
    Flush,
    #[serde(rename = "full")]
    Full,
}

/// The horizontal alignment of the text. One of `"left"`, `"right"`, `"center"`.
///
/// Horizontal text alignment of axis tick labels, overriding the default setting for the
/// current axis orientation.
///
/// Horizontal text alignment of axis titles.
///
/// The alignment of the legend label, can be left, center, or right.
///
/// Horizontal text alignment for legend titles.
///
/// __Default value:__ `"left"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Align {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

/// The vertical alignment of the text. One of `"top"`, `"middle"`, `"bottom"`.
///
/// __Default value:__ `"middle"`
///
/// Vertical text baseline of axis tick labels, overriding the default setting for the
/// current axis orientation. Can be `"top"`, `"middle"`, `"bottom"`, or `"alphabetic"`.
///
/// Vertical text baseline for axis titles.
///
/// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
///
/// __Default value:__ `"middle"`
///
/// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
/// `"alphabetic"`.
///
/// __Default value:__ `"middle"`.
///
/// Vertical text baseline for legend titles.
///
/// __Default value:__ `"top"`.
///
/// Vertical text baseline for title text. One of `"top"`, `"middle"`, `"bottom"`, or
/// `"alphabetic"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextBaseline {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "top")]
    Top,
}

/// The mouse cursor used over the mark. Any valid [CSS cursor
/// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Cursor {
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "all-scroll")]
    AllScroll,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "cell")]
    Cell,
    #[serde(rename = "col-resize")]
    ColResize,
    #[serde(rename = "context-menu")]
    ContextMenu,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "crosshair")]
    Crosshair,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "e-resize")]
    EResize,
    #[serde(rename = "ew-resize")]
    EwResize,
    #[serde(rename = "grab")]
    Grab,
    #[serde(rename = "grabbing")]
    Grabbing,
    #[serde(rename = "help")]
    Help,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "n-resize")]
    NResize,
    #[serde(rename = "ne-resize")]
    NeResize,
    #[serde(rename = "nesw-resize")]
    NeswResize,
    #[serde(rename = "no-drop")]
    NoDrop,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "not-allowed")]
    NotAllowed,
    #[serde(rename = "ns-resize")]
    NsResize,
    #[serde(rename = "nw-resize")]
    NwResize,
    #[serde(rename = "nwse-resize")]
    NwseResize,
    #[serde(rename = "pointer")]
    Pointer,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "row-resize")]
    RowResize,
    #[serde(rename = "s-resize")]
    SResize,
    #[serde(rename = "se-resize")]
    SeResize,
    #[serde(rename = "sw-resize")]
    SwResize,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "vertical-text")]
    VerticalText,
    #[serde(rename = "w-resize")]
    WResize,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "zoom-in")]
    ZoomIn,
    #[serde(rename = "zoom-out")]
    ZoomOut,
}

/// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
/// This property determines on which side is truncated in response to the limit parameter.
///
/// __Default value:__ `"ltr"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Dir {
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "rtl")]
    Rtl,
}

/// The font style (e.g., `"italic"`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FontStyle {
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FontWeightEnum {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "bolder")]
    Bolder,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "normal")]
    Normal,
}

/// The line interpolation method to use for line and area marks. One of the following:
/// - `"linear"`: piecewise linear segments, as in a polyline.
/// - `"linear-closed"`: close the linear segments to form a polygon.
/// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
/// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
/// function.
/// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
/// function.
/// - `"basis"`: a B-spline, with control point duplication on the ends.
/// - `"basis-open"`: an open B-spline; may not intersect the start or end.
/// - `"basis-closed"`: a closed B-spline, as in a loop.
/// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
/// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
/// will intersect other control points.
/// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
/// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
/// spline.
/// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
///
/// The line interpolation method for the error band. One of the following:
/// - `"linear"`: piecewise linear segments, as in a polyline.
/// - `"linear-closed"`: close the linear segments to form a polygon.
/// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
/// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
/// function.
/// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
/// function.
/// - `"basis"`: a B-spline, with control point duplication on the ends.
/// - `"basis-open"`: an open B-spline; may not intersect the start or end.
/// - `"basis-closed"`: a closed B-spline, as in a loop.
/// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
/// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
/// will intersect other control points.
/// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
/// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
/// spline.
/// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Interpolate {
    #[serde(rename = "basis")]
    Basis,
    #[serde(rename = "basis-closed")]
    BasisClosed,
    #[serde(rename = "basis-open")]
    BasisOpen,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "cardinal")]
    Cardinal,
    #[serde(rename = "cardinal-closed")]
    CardinalClosed,
    #[serde(rename = "cardinal-open")]
    CardinalOpen,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "linear-closed")]
    LinearClosed,
    #[serde(rename = "monotone")]
    Monotone,
    #[serde(rename = "step")]
    Step,
    #[serde(rename = "step-after")]
    StepAfter,
    #[serde(rename = "step-before")]
    StepBefore,
}

/// The orientation of a non-stacked bar, tick, area, and line charts.
/// The value is either horizontal (default) or vertical.
/// - For bar, rule and tick, this determines whether the size of the bar and tick
/// should be applied to x or y dimension.
/// - For area, this property determines the orient property of the Vega output.
/// - For line and trail marks, this property determines the sort order of the points in the
/// line
/// if `config.sortLineBy` is not specified.
/// For stacked charts, this is always determined by the orientation of the stack;
/// therefore explicitly specified value will be ignored.
///
/// Orientation of the box plot.  This is normally automatically determined based on types of
/// fields on x and y channels. However, an explicit `orient` be specified when the
/// orientation is ambiguous.
///
/// __Default value:__ `"vertical"`.
///
/// Orientation of the error bar.  This is normally automatically determined, but can be
/// specified when the orientation is ambiguous and cannot be automatically determined.
///
/// Orientation of the error band. This is normally automatically determined, but can be
/// specified when the orientation is ambiguous and cannot be automatically determined.
///
/// The default direction (`"horizontal"` or `"vertical"`) for gradient legends.
///
/// __Default value:__ `"vertical"`.
///
/// The default direction (`"horizontal"` or `"vertical"`) for symbol legends.
///
/// __Default value:__ `"vertical"`.
///
/// The direction of the legend, one of `"vertical"` (default) or `"horizontal"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Orient {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

/// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
///
/// __Default value:__ `"square"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrokeCap {
    #[serde(rename = "butt")]
    Butt,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "square")]
    Square,
}

/// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
///
/// __Default value:__ `"miter"`
///
/// The stroke line join method. One of miter (default), round or bevel.
///
/// __Default value:__ 'miter'
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrokeJoin {
    #[serde(rename = "bevel")]
    Bevel,
    #[serde(rename = "miter")]
    Miter,
    #[serde(rename = "round")]
    Round,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Content {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "encoding")]
    Encoding,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PointEnum {
    #[serde(rename = "transparent")]
    Transparent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LabelOverlapEnum {
    #[serde(rename = "greedy")]
    Greedy,
    #[serde(rename = "parity")]
    Parity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtentEnum {
    #[serde(rename = "min-max")]
    MinMax,
}

/// The extent of the band. Available options include:
/// - `"ci"`: Extend the band to the confidence interval of the mean.
/// - `"stderr"`: The size of band are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the band to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
///
/// The extent of the rule. Available options include:
/// - `"ci"`: Extend the rule to the confidence interval of the mean.
/// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the rule to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ErrorbandExtent {
    #[serde(rename = "ci")]
    Ci,
    #[serde(rename = "iqr")]
    Iqr,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
}

/// Defines how Vega-Lite generates title for fields.  There are three possible styles:
/// - `"verbal"` (Default) - displays function in a verbal style (e.g., "Sum of field",
/// "Year-month of date", "field (binned)").
/// - `"function"` - displays function using parentheses and capitalized texts (e.g.,
/// "SUM(field)", "YEARMONTH(date)", "BIN(field)").
/// - `"plain"` - displays only the field name without functions (e.g., "field", "date",
/// "field").
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FieldTitle {
    #[serde(rename = "functional")]
    Functional,
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "verbal")]
    Verbal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InvalidValues {
    #[serde(rename = "filter")]
    Filter,
}

/// The orientation of the legend, which determines how the legend is positioned within the
/// scene. One of "left", "right", "top-left", "top-right", "bottom-left", "bottom-right",
/// "none".
///
/// __Default value:__ `"right"`
///
/// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
///
/// The orientation of the axis. One of `"top"`, `"bottom"`, `"left"` or `"right"`. The
/// orientation can be used to further specialize the axis type (e.g., a y axis oriented for
/// the right edge of the chart).
///
/// __Default value:__ `"bottom"` for x-axes and `"left"` for y-axes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LegendOrient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
}

/// The cartographic projection to use. This value is case-insensitive, for example
/// `"albers"` and `"Albers"` indicate the same projection type. You can find all valid
/// projection types [in the
/// documentation](https://vega.github.io/vega-lite/docs/projection.html#projection-types).
///
/// __Default value:__ `mercator`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VgProjectionType {
    #[serde(rename = "albers")]
    Albers,
    #[serde(rename = "albersUsa")]
    AlbersUsa,
    #[serde(rename = "azimuthalEqualArea")]
    AzimuthalEqualArea,
    #[serde(rename = "azimuthalEquidistant")]
    AzimuthalEquidistant,
    #[serde(rename = "conicConformal")]
    ConicConformal,
    #[serde(rename = "conicEqualArea")]
    ConicEqualArea,
    #[serde(rename = "conicEquidistant")]
    ConicEquidistant,
    #[serde(rename = "equirectangular")]
    Equirectangular,
    #[serde(rename = "gnomonic")]
    Gnomonic,
    #[serde(rename = "mercator")]
    Mercator,
    #[serde(rename = "orthographic")]
    Orthographic,
    #[serde(rename = "stereographic")]
    Stereographic,
    #[serde(rename = "transverseMercator")]
    TransverseMercator,
}

/// Establishes a two-way binding between the interval selection and the scales
/// used within the same view. This allows a user to interactively pan and
/// zoom the view.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BindEnum {
    #[serde(rename = "scales")]
    Scales,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SingleDefChannel {
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "href")]
    Href,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "latitude")]
    Latitude,
    #[serde(rename = "latitude2")]
    Latitude2,
    #[serde(rename = "longitude")]
    Longitude,
    #[serde(rename = "longitude2")]
    Longitude2,
    #[serde(rename = "opacity")]
    Opacity,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "stroke")]
    Stroke,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "tooltip")]
    Tooltip,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "x2")]
    X2,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "y2")]
    Y2,
}

/// With layered and multi-view displays, a strategy that determines how
/// selections' data queries are resolved when applied in a filter transform,
/// conditional encoding rule, or scale domain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SelectionResolution {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "intersect")]
    Intersect,
    #[serde(rename = "union")]
    Union,
}

/// Default stack offset for stackable mark.
///
/// Mode for stacking marks.
/// __Default value:__ `"zero"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StackOffset {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "normalize")]
    Normalize,
    #[serde(rename = "zero")]
    Zero,
}

/// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
/// example, with an orientation of top these anchor positions map to a left-, center-, or
/// right-aligned title.
///
/// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
/// example, with an orientation of top these anchor positions map to a left-, center-, or
/// right-aligned title.
///
/// __Default value:__ `"middle"` for
/// [single](https://vega.github.io/vega-lite/docs/spec.html) and
/// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.
/// `"start"` for other composite views.
///
/// __Note:__ [For now](https://github.com/vega/vega-lite/issues/2875), `anchor` is only
/// customizable only for [single](https://vega.github.io/vega-lite/docs/spec.html) and
/// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.  For other composite
/// views, `anchor` is always `"start"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TitleAnchor {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "start")]
    Start,
}

/// The reference frame for the anchor position, one of `"bounds"` (to anchor relative to the
/// full bounding box) or `"group"` (to anchor relative to the group width or height).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TitleFrame {
    #[serde(rename = "bounds")]
    Bounds,
    #[serde(rename = "group")]
    Group,
}

/// Type of input data: `"json"`, `"csv"`, `"tsv"`, `"dsv"`.
/// The default format type is determined by the extension of the file URL.
/// If no extension is detected, `"json"` will be used by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataFormatType {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "dsv")]
    Dsv,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "topojson")]
    Topojson,
    #[serde(rename = "tsv")]
    Tsv,
}

/// Aggregation function for the field
/// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
///
/// __Default value:__ `undefined` (None)
///
/// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
/// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
/// This property is required in cases where the sort field and the data reference field do
/// not match.
/// The input data objects will be aggregated, grouped by the encoded data field.
///
/// For a full list of operations, please see the documentation for
/// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
///
/// The aggregation operations to apply to the fields, such as sum, average or count.
/// See the [full list of supported aggregation
/// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
/// for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AggregateOp {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BinEnum {
    #[serde(rename = "binned")]
    Binned,
}

/// Time unit for the field to be filtered.
///
/// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
/// or [a temporal field that gets casted as
/// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
///
/// __Default value:__ `undefined` (None)
///
/// The timeUnit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TimeUnit {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "hoursminutes")]
    Hoursminutes,
    #[serde(rename = "hoursminutesseconds")]
    Hoursminutesseconds,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "minutesseconds")]
    Minutesseconds,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "monthdate")]
    Monthdate,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "quartermonth")]
    Quartermonth,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "secondsmilliseconds")]
    Secondsmilliseconds,
    #[serde(rename = "utcdate")]
    Utcdate,
    #[serde(rename = "utcday")]
    Utcday,
    #[serde(rename = "utchours")]
    Utchours,
    #[serde(rename = "utchoursminutes")]
    Utchoursminutes,
    #[serde(rename = "utchoursminutesseconds")]
    Utchoursminutesseconds,
    #[serde(rename = "utcmilliseconds")]
    Utcmilliseconds,
    #[serde(rename = "utcminutes")]
    Utcminutes,
    #[serde(rename = "utcminutesseconds")]
    Utcminutesseconds,
    #[serde(rename = "utcmonth")]
    Utcmonth,
    #[serde(rename = "utcmonthdate")]
    Utcmonthdate,
    #[serde(rename = "utcquarter")]
    Utcquarter,
    #[serde(rename = "utcquartermonth")]
    Utcquartermonth,
    #[serde(rename = "utcseconds")]
    Utcseconds,
    #[serde(rename = "utcsecondsmilliseconds")]
    Utcsecondsmilliseconds,
    #[serde(rename = "utcyear")]
    Utcyear,
    #[serde(rename = "utcyearmonth")]
    Utcyearmonth,
    #[serde(rename = "utcyearmonthdate")]
    Utcyearmonthdate,
    #[serde(rename = "utcyearmonthdatehours")]
    Utcyearmonthdatehours,
    #[serde(rename = "utcyearmonthdatehoursminutes")]
    Utcyearmonthdatehoursminutes,
    #[serde(rename = "utcyearmonthdatehoursminutesseconds")]
    Utcyearmonthdatehoursminutesseconds,
    #[serde(rename = "utcyearquarter")]
    Utcyearquarter,
    #[serde(rename = "utcyearquartermonth")]
    Utcyearquartermonth,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "yearmonth")]
    Yearmonth,
    #[serde(rename = "yearmonthdate")]
    Yearmonthdate,
    #[serde(rename = "yearmonthdatehours")]
    Yearmonthdatehours,
    #[serde(rename = "yearmonthdatehoursminutes")]
    Yearmonthdatehoursminutes,
    #[serde(rename = "yearmonthdatehoursminutesseconds")]
    Yearmonthdatehoursminutesseconds,
    #[serde(rename = "yearquarter")]
    Yearquarter,
    #[serde(rename = "yearquartermonth")]
    Yearquartermonth,
}

/// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
/// `"nominal"`).
/// It can also be a `"geojson"` type for encoding
/// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
///
/// Constants and utilities for data type
/// Data type based on level of measurement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "geojson")]
    Geojson,
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "quantitative")]
    Quantitative,
    #[serde(rename = "temporal")]
    Temporal,
}

/// The type of the legend. Use `"symbol"` to create a discrete legend and `"gradient"` for a
/// continuous color gradient.
///
/// __Default value:__ `"gradient"` for non-binned quantitative fields and temporal fields;
/// `"symbol"` otherwise.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LegendType {
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "symbol")]
    Symbol,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Domain {
    #[serde(rename = "unaggregated")]
    Unaggregated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScaleInterpolateParamsType {
    #[serde(rename = "cubehelix")]
    Cubehelix,
    #[serde(rename = "cubehelix-long")]
    CubehelixLong,
    #[serde(rename = "rgb")]
    Rgb,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScaleInterpolate {
    #[serde(rename = "cubehelix")]
    Cubehelix,
    #[serde(rename = "cubehelix-long")]
    CubehelixLong,
    #[serde(rename = "hcl")]
    Hcl,
    #[serde(rename = "hcl-long")]
    HclLong,
    #[serde(rename = "hsl")]
    Hsl,
    #[serde(rename = "hsl-long")]
    HslLong,
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "rgb")]
    Rgb,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NiceTime {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

/// The type of scale.  Vega-Lite supports the following categories of scale types:
///
/// 1) [**Continuous Scales**](https://vega.github.io/vega-lite/docs/scale.html#continuous)
/// -- mapping continuous domains to continuous output ranges
/// ([`"linear"`](https://vega.github.io/vega-lite/docs/scale.html#linear),
/// [`"pow"`](https://vega.github.io/vega-lite/docs/scale.html#pow),
/// [`"sqrt"`](https://vega.github.io/vega-lite/docs/scale.html#sqrt),
/// [`"log"`](https://vega.github.io/vega-lite/docs/scale.html#log),
/// [`"time"`](https://vega.github.io/vega-lite/docs/scale.html#time),
/// [`"utc"`](https://vega.github.io/vega-lite/docs/scale.html#utc),
/// [`"sequential"`](https://vega.github.io/vega-lite/docs/scale.html#sequential)).
///
/// 2) [**Discrete Scales**](https://vega.github.io/vega-lite/docs/scale.html#discrete) --
/// mapping discrete domains to discrete
/// ([`"ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#ordinal)) or continuous
/// ([`"band"`](https://vega.github.io/vega-lite/docs/scale.html#band) and
/// [`"point"`](https://vega.github.io/vega-lite/docs/scale.html#point)) output ranges.
///
/// 3) [**Discretizing
/// Scales**](https://vega.github.io/vega-lite/docs/scale.html#discretizing) -- mapping
/// continuous domains to discrete output ranges
/// ([`"bin-linear"`](https://vega.github.io/vega-lite/docs/scale.html#bin-linear),
/// [`"bin-ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#bin-ordinal),
/// [`"quantile"`](https://vega.github.io/vega-lite/docs/scale.html#quantile),
/// [`"quantize"`](https://vega.github.io/vega-lite/docs/scale.html#quantize) and
/// [`"threshold"`](https://vega.github.io/vega-lite/docs/scale.html#threshold).
///
/// __Default value:__ please see the [scale type
/// table](https://vega.github.io/vega-lite/docs/scale.html#type).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ScaleType {
    #[serde(rename = "band")]
    Band,
    #[serde(rename = "bin-linear")]
    BinLinear,
    #[serde(rename = "bin-ordinal")]
    BinOrdinal,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "pow")]
    Pow,
    #[serde(rename = "quantile")]
    Quantile,
    #[serde(rename = "quantize")]
    Quantize,
    #[serde(rename = "sequential")]
    Sequential,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "threshold")]
    Threshold,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "utc")]
    Utc,
}

/// Whether to sort the field in ascending or descending order.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VgComparatorOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

/// The imputation method to use for the field value of imputed data objects.
/// One of `value`, `mean`, `median`, `max` or `min`.
///
/// __Default value:__  `"value"`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImputeMethod {
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "value")]
    Value,
}

/// The mark type. This could a primitive mark type
/// (one of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
/// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`)
/// or a composite mark type (`"boxplot"`, `"errorband"`, `"errorbar"`).
///
/// All types of primitive marks.
///
/// The mark type.
/// One of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
/// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BoxPlot {
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "boxplot")]
    Boxplot,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "errorband")]
    Errorband,
    #[serde(rename = "errorbar")]
    Errorbar,
    #[serde(rename = "geoshape")]
    Geoshape,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "trail")]
    Trail,
}

/// The extent of the band. Available options include:
/// - `"ci"`: Extend the band to the confidence interval of the mean.
/// - `"stderr"`: The size of band are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the band to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
///
/// The extent of the rule. Available options include:
/// - `"ci"`: Extend the rule to the confidence interval of the mean.
/// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the rule to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExtentExtent {
    #[serde(rename = "ci")]
    Ci,
    #[serde(rename = "iqr")]
    Iqr,
    #[serde(rename = "min-max")]
    MinMax,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResolveMode {
    #[serde(rename = "independent")]
    Independent,
    #[serde(rename = "shared")]
    Shared,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SelectionDefType {
    #[serde(rename = "interval")]
    Interval,
    #[serde(rename = "multi")]
    Multi,
    #[serde(rename = "single")]
    Single,
}

/// The window or aggregation operations to apply within a window, including `rank`, `lead`,
/// `sum`, `average` or `count`. See the list of all supported operations
/// [here](https://vega.github.io/vega-lite/docs/window.html#ops).
///
/// Aggregation function for the field
/// (e.g., `mean`, `sum`, `median`, `min`, `max`, `count`).
///
/// __Default value:__ `undefined` (None)
///
/// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
/// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
/// This property is required in cases where the sort field and the data reference field do
/// not match.
/// The input data objects will be aggregated, grouped by the encoded data field.
///
/// For a full list of operations, please see the documentation for
/// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
///
/// The aggregation operations to apply to the fields, such as sum, average or count.
/// See the [full list of supported aggregation
/// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
/// for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "cume_dist")]
    CumeDist,
    #[serde(rename = "dense_rank")]
    DenseRank,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "first_value")]
    FirstValue,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "last_value")]
    LastValue,
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "nth_value")]
    NthValue,
    #[serde(rename = "ntile")]
    Ntile,
    #[serde(rename = "percent_rank")]
    PercentRank,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "rank")]
    Rank,
    #[serde(rename = "row_number")]
    RowNumber,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}
