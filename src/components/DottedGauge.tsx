import type { DotColor } from "../preferences";

interface DottedGaugeProps {
  value: number | null;
  label: string;
  displayValue?: string | null;
  valueKind?: "default" | "duration";
  dotColor?: DotColor;
}

const DOT_COUNT = 42;
// Geometry never changes with quota, clock ticks, or preferences.
const DOTS = Array.from({ length: DOT_COUNT }, (_, index) => {
  const angle = (index / DOT_COUNT) * Math.PI * 2 - Math.PI / 2;
  return { x: 115 + Math.cos(angle) * 96, y: 115 + Math.sin(angle) * 96 };
});

export function DottedGauge({ value, label, displayValue, valueKind = "default", dotColor = "black" }: DottedGaugeProps) {
  const activeDots = value === null
    ? 0
    : Math.round((Math.min(100, Math.max(0, value)) / 100) * DOT_COUNT);
  const shownValue = displayValue ?? (value === null ? "--" : `${value}%`);
  const isPercentage = shownValue.endsWith("%");
  const number = isPercentage ? shownValue.slice(0, -1) : shownValue;
  const ariaValue = `${shownValue} ${label}`;
  const isDuration = valueKind === "duration";
  const activeColor = value !== null && value <= 10 ? "critical" : dotColor;

  return (
    <div className={`gauge${isDuration ? " gauge--duration" : ""}`} data-dot-color={activeColor} role="img" aria-label={ariaValue}>
      <div className="gauge__ring-layer" hidden={isDuration}>
        <svg className="gauge__ring" viewBox="0 0 230 230" aria-hidden="true">
          {DOTS.map(({ x, y }, index) => {
            const isActive = index < activeDots;
            return (
              <circle
                key={index}
                cx={x}
                cy={y}
                r="5.2"
                className={isActive ? "gauge__dot gauge__dot--active" : "gauge__dot"}
              />
            );
          })}
        </svg>
      </div>
      <div className="gauge__content">
        <div className={`gauge__value${value === null ? " gauge__value--empty" : ""}${valueKind === "duration" ? " gauge__value--duration" : ""}`}>
          <span className="gauge__value-number" key={`${valueKind}:${number}`}>{number}</span>
          {isPercentage ? <span className="gauge__value-unit">%</span> : null}
        </div>
        <div className="gauge__label" hidden={isDuration}>{label}</div>
      </div>
    </div>
  );
}
