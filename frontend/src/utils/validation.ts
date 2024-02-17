export function validateInput(
  value: string | number | null,
  minValue: number,
  maxValue: number,
  decimalPoints: number,
): string {
  if (Number.isNaN(Number(value))) return "Invalid amount";

  if (value && Number(value < minValue))
    return `Amount too small. Min = ${minValue}`;

  if (value && Number(value) > maxValue)
    return `Amount too big. Max = ${maxValue}`;

  if (
    value &&
    Number(value).toFixed(decimalPoints).length < value.toString().length
  )
    return `Max ${decimalPoints} decimal points are allowed`;

  return "";
}
