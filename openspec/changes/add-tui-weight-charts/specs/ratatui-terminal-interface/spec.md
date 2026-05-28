## ADDED Requirements

### Requirement: TUI renders a Summary weight trend chart
The TUI SHALL render a weight trend chart in the Summary analysis view when enough data and space are available.

#### Scenario: Summary chart data is available
- **WHEN** the Summary analysis view is rendered with at least two recent weight records and sufficient panel space
- **THEN** the Summary analysis view displays a weight trend chart derived from the recent records
- **THEN** the Summary textual trend, BMI, and TDEE details remain visible

#### Scenario: Summary chart cannot be shown
- **WHEN** the Summary analysis view has fewer than two records or insufficient panel space
- **THEN** the Summary analysis view keeps its text-only analysis visible
- **THEN** the TUI does not render a misleading empty chart

### Requirement: TUI keeps non-trend analysis panels text-first
The TUI SHALL avoid adding charts to analysis panels where the available data is not naturally chart-oriented.

#### Scenario: Recent records panel is rendered
- **WHEN** the TUI renders the Recent records panel
- **THEN** the panel remains text-first without adding a chart

#### Scenario: Advice view is rendered
- **WHEN** the TUI renders the Advice analysis view
- **THEN** the view remains text-first without adding a chart

#### Scenario: Target view is rendered
- **WHEN** the TUI renders the Target analysis view
- **THEN** the view remains text-first without adding a chart

#### Scenario: Compare view is rendered
- **WHEN** the TUI renders the Compare analysis view
- **THEN** the view remains table-first without adding a chart in this change
