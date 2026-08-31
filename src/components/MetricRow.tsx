import { Icon, type IconName } from "./Icon";

interface MetricRowProps {
  icon: IconName;
  label: string;
  value: string;
  developmentOnly?: boolean;
}

export function MetricRow({ icon, label, value, developmentOnly = false }: MetricRowProps) {
  return (
    <div className="metric-row">
      <span className="metric-row__icon"><Icon name={icon} /></span>
      <span className="metric-row__label">
        {label}
        {developmentOnly ? <span className="dev-mark" aria-label="development mock">DEV</span> : null}
      </span>
      <span className="metric-row__value">{value}</span>
    </div>
  );
}
