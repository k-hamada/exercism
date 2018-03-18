defmodule Wordy do
  @doc """
  Calculate the math problem in the sentence.
  """
  @spec answer(String.t()) :: integer
  def answer(question) do
    Regex.scan(~r/\w+|\-?\d+|\?/, question)
    |> List.flatten()
    |> do_answer()
  end

  def do_answer(["What", "is" | question]) do
    question
    |> do_answer()
  end

  def do_answer([n, "plus", m | question]) do
    [calc(:+, n, m) | question]
    |> do_answer()
  end

  def do_answer([n, "minus", m | question]) do
    [calc(:-, n, m) | question]
    |> do_answer()
  end

  def do_answer([n, "multiplied", "by", m | question]) do
    [calc(:*, n, m) | question]
    |> do_answer()
  end

  def do_answer([n, "divided", "by", m | question]) do
    [calc(:div, n, m) | question]
    |> do_answer()
  end

  def do_answer([n, "?"]) do
    n |> String.to_integer()
  end

  def do_answer(_) do
    raise ArgumentError
  end

  def calc(op, n, m) do
    apply(Kernel, op, [n, m] |> Enum.map(&String.to_integer/1))
    |> to_string()
  end
end
