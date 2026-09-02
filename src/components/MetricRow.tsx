import { type IconName } from "./Icon";
import { MetricIcon } from "./MetricIcon";

interface MetricRowProps {
  icon: IconName;
  label: string;
  value: string;
}

export function MetricRow({ icon, label, value }: MetricRowProps) {
  return (
    <div className="metric-row">
      <MetricIcon name={icon} />
      <span className="metric-row__label">{label}</span>
      <span className="metric-row__value">{value}</span>
    </div>
  );
}
