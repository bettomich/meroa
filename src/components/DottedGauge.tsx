interface DottedGaugeProps {
  value: number | null;
  label: string;
  displayValue?: string | null;
  valueKind?: "default" | "duration";
}

const DOT_COUNT = 42;

export function DottedGauge({ value, label, displayValue, valueKind = "default" }: DottedGaugeProps) {
  const activeDots = value === null
    ? 0
    : Math.round((Math.min(100, Math.max(0, value)) / 100) * DOT_COUNT);
  const shownValue = displayValue ?? (value === null ? "--" : `${value}%`);
  const isPercentage = shownValue.endsWith("%");
  const number = isPercentage ? shownValue.slice(0, -1) : shownValue;
  const ariaValue = `${shownValue} ${label}`;

  return (
    <div className="gauge" role="img" aria-label={ariaValue}>
      <svg className="gauge__ring" viewBox="0 0 230 230" aria-hidden="true">
        {Array.from({ length: DOT_COUNT }, (_, index) => {
          const angle = (index / DOT_COUNT) * Math.PI * 2 - Math.PI / 2;
          const x = 115 + Math.cos(angle) * 96;
          const y = 115 + Math.sin(angle) * 96;
          const isActive = index < activeDots;
          const isAccent = isActive && index >= Math.max(0, activeDots - 3);
          return (
            <circle
              key={index}
              cx={x}
              cy={y}
              r="5.2"
              className={isAccent ? "gauge__dot gauge__dot--accent" : isActive ? "gauge__dot gauge__dot--active" : "gauge__dot"}
            />
          );
        })}
      </svg>
      <div className="gauge__content">
        <div className={`gauge__value${value === null ? " gauge__value--empty" : ""}${valueKind === "duration" ? " gauge__value--duration" : ""}`}>
          <span className="gauge__value-number">{number}</span>
          {isPercentage ? <span className="gauge__value-unit">%</span> : null}
        </div>
        <div className="gauge__label">{label}</div>
      </div>
    </div>
  );
}
