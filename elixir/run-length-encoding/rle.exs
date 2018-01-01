defmodule RunLengthEncoder do
  @doc """
  Generates a string where consecutive elements are represented as a data value and count.
  "AABBBCCCC" => "2A3B4C"
  For this example, assume all input are strings, that are all uppercase letters.
  It should also be able to reconstruct the data into its original form.
  "2A3B4C" => "AABBBCCCC"
  """
  @spec encode(String.t) :: String.t
  def encode(string) do
    string
    |> String.graphemes
    |> Enum.chunk_by(&(&1))
    |> Enum.map(&to_count_and_word/1)
    |> Enum.map_join(&contract/1)
  end

  @spec decode(String.t) :: String.t
  def decode(string) do
    string
    |> from_count_and_word(&String.duplicate/2)
  end

  defp to_count_and_word(strings) do
    [Enum.count(strings), hd(strings)]
  end

  defp contract([1, string]) do
    string
  end

  defp contract([count, string]) do
    to_string(count) <> string
  end

  defp from_count_and_word(string, fun) do
    Regex.replace(~r/(\d+)(.)/, string, fn _, count, word -> fun.(word, String.to_integer(count)) end)
  end
end
