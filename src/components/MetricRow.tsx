import { type IconName } from "./Icon";
import { MetricIcon } from "./MetricIcon";

interface MetricRowProps {
  icon: IconName;
  label: string;
  value: string;
  selected: boolean;
  onSelect: () => void;
}

function MetricValue({ value }: { value: string }) {
  const isPercentage = value.endsWith("%");
  const number = isPercentage ? value.slice(0, -1) : value;

  return (
    <span className="metric-row__value">
      <span className="metric-row__value-number">{number}</span>
      {isPercentage ? <span className="metric-row__value-unit">%</span> : null}
    </span>
  );
}

export function MetricRow({ icon, label, value, selected, onSelect }: MetricRowProps) {
  return (
    <button
      className={selected ? "metric-row is-selected" : "metric-row"}
      type="button"
      aria-pressed={selected}
      onClick={onSelect}
    >
      <MetricIcon name={icon} />
      <span className="metric-row__label">{label}</span>
      <MetricValue value={value} />
    </button>
  );
}
