import { Icon, type IconName } from "./Icon";

interface MetricIconProps {
  name: IconName;
}

export function MetricIcon({ name }: MetricIconProps) {
  return (
    <span className="metric-icon" aria-hidden="true">
      <Icon name={name} />
    </span>
  );
}
