package erratum

func Use(o ResourceOpener, input string) (err error) {
	var res Resource
	for {
		res, err = o()
		if err == nil {
			break
		}
		if _, ok := err.(TransientError); ok {
			continue
		}
		return
	}
	defer res.Close()
	defer func() {
		if rec := recover(); rec != nil {
			if frob, ok := rec.(FrobError); ok {
				res.Defrob(frob.defrobTag)
			}
			err = rec.(error)
		}
	}()
	res.Frob(input)
	return
}
