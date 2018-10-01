defmodule Phone do
  @doc """
  Remove formatting from a phone number.

  Returns "0000000000" if phone number is not valid
  (10 digits or "1" followed by 10 digits)

  ## Examples

  iex> Phone.number("212-555-0100")
  "2125550100"

  iex> Phone.number("+1 (212) 555-0100")
  "2125550100"

  iex> Phone.number("+1 (212) 055-0100")
  "0000000000"

  iex> Phone.number("(212) 555-0100")
  "2125550100"

  iex> Phone.number("867.5309")
  "0000000000"
  """
  @spec number(String.t) :: String.t
  def number(raw) do
    Regex.scan(~r/\+?(\d{0,1}).*(\d{3}).*(\d{3}).*(\d{4})/, raw, capture: :all_but_first)
    |> List.flatten
    |> Enum.flat_map(&String.graphemes/1)
    |> Enum.map(&String.to_integer/1)
    |> format
    |> Enum.join
  end

  defp format([1 | number]) do
    format(number)
  end

  defp format([a, b, c, d, e, f, g, h, i, j]) when a in 2..9 and d in 2..9 do
    [a, b, c, d, e, f, g, h, i, j]
  end

  defp format(_) do
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  end

  @doc """
  Extract the area code from a phone number

  Returns the first three digits from a phone number,
  ignoring long distance indicator

  ## Examples

  iex> Phone.area_code("212-555-0100")
  "212"

  iex> Phone.area_code("+1 (212) 555-0100")
  "212"

  iex> Phone.area_code("+1 (012) 555-0100")
  "000"

  iex> Phone.area_code("867.5309")
  "000"
  """
  @spec area_code(String.t) :: String.t
  def area_code(raw) do
    raw
    |> number
    |> String.slice(0..2)
  end

  @doc """
  Pretty print a phone number

  Wraps the area code in parentheses and separates
  exchange and subscriber number with a dash.

  ## Examples

  iex> Phone.pretty("212-555-0100")
  "(212) 555-0100"

  iex> Phone.pretty("212-155-0100")
  "(000) 000-0000"

  iex> Phone.pretty("+1 (303) 555-1212")
  "(303) 555-1212"

  iex> Phone.pretty("867.5309")
  "(000) 000-0000"
  """
  @spec pretty(String.t) :: String.t
  def pretty(raw) do
    raw
    |> number
    |> String.replace(~r/(\d{3})(\d{3})(\d{4})/, "(\\1) \\2-\\3")
  end
end
