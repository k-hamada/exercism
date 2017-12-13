defmodule SpaceAge do
  @type planet :: :mercury | :venus | :earth | :mars | :jupiter
                | :saturn | :uranus | :neptune

  @earth_second 31_557_600
  @planets %{
  	mercury: 0.2408467,
  	venus:   0.6151972,
  	earth:   1,
  	mars:    1.8808158,
  	jupiter: 11.862615,
  	saturn:  29.447498,
  	uranus:  84.016846,
  	neptune: 164.79132,
  }

  @doc """
  Return the number of years a person that has lived for 'seconds' seconds is
  aged on 'planet'.
  """
  @spec age_on(planet, pos_integer) :: float
  def age_on(planet, seconds) do
    case Map.fetch(@planets, planet) do
      {:ok, planet_second} -> seconds / (planet_second * @earth_second)
      :error -> 0
    end
  end
end
