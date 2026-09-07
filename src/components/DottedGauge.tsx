interface DottedGaugeProps {
  value: number | null;
  label: string;
  displayValue?: string | null;
}

const DOT_COUNT = 42;

export function DottedGauge({ value, label, displayValue }: DottedGaugeProps) {
  const activeDots = value === null
    ? 0
    : Math.round((Math.min(100, Math.max(0, value)) / 100) * DOT_COUNT);
  const shownValue = displayValue ?? (value === null ? "--" : `${value}%`);
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
        <div className={value === null ? "gauge__value gauge__value--empty" : "gauge__value"}>
          <span>{shownValue}</span>
        </div>
        <div className="gauge__label">{label}</div>
      </div>
    </div>
  );
}
