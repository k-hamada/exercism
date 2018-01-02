defmodule Scrabble do
  @scoredoc ~s'''
    A, E, I, O, U, L, N, R, S, T       1
    D, G                               2
    B, C, M, P                         3
    F, H, V, W, Y                      4
    K                                  5
    J, X                               8
    Q, Z                               10
  '''

  @doc """
  Calculate the scrabble score for the word.
  """
  @spec score(String.t) :: non_neg_integer
  def score(word) do
    scoremap = make_scoremap()
    word
    |> String.graphemes
    |> Enum.map(&String.upcase/1)
    |> Enum.map(fn(w) -> Map.get(scoremap, w, 0) end)
    |> Enum.sum
  end

  defp make_scoremap() do
    @scoredoc
    |> String.split(~r/\n/)
    |> Enum.map(fn(line) -> Regex.named_captures(~r/(?<words>(?:\w,\s)+\w|\w).+?(?<score>\d+)/, line) end)
    |> Enum.reject(&is_nil/1)
    |> Enum.flat_map(fn(m) -> m["words"] |> String.split(~r/,\s/) |> Enum.map(&({&1, String.to_integer(m["score"])})) end)
    |> Map.new
  end
end
