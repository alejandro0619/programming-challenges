function goodVsEvil(good: string, evil: string): string {
  // constanst
  const HOBBITS_WORTH = 1;
  const MEN_WORTH = 2;
  const ELVES_WORTH = 3;
  const DWARVES_WORTH = 3;
  const EAGLES_WORTH = 4;
  const WIZARDS_WORTH = 10;
  const ORCS_WORTH = 1;
  const WARGS_WORTH = 2;
  const GOBLINS_WORTH = 2;
  const URUKHAI_WORTH = 3;
  const TROLLS_WORTH = 5;

  const goodParsed: number[] = good.split(' ').map(race => parseInt(race));
  const evilParsed: number[] = evil.split(' ').map(race => parseInt(race));

  const goodWorth: number =
    goodParsed[0] * HOBBITS_WORTH +
    goodParsed[1] * MEN_WORTH +
    goodParsed[2] * ELVES_WORTH +
    goodParsed[3] * DWARVES_WORTH +
    goodParsed[4] * EAGLES_WORTH +
    goodParsed[5] * WIZARDS_WORTH;
  
    const evilWorth: number =
    evilParsed[0] * ORCS_WORTH +
    evilParsed[1] * MEN_WORTH +
    evilParsed[2] * WARGS_WORTH +
    evilParsed[3] * GOBLINS_WORTH +
    evilParsed[4] * URUKHAI_WORTH +
    evilParsed[5] * TROLLS_WORTH + 
    evilParsed[6] * WIZARDS_WORTH
  
  if (goodWorth > evilWorth) {
    return 'Good triumphs over Evil" if good wins'
  } else if (goodWorth == evilWorth) {
    return 'No victor on this battle field'
  } else {
    return 'Evil eradicates all trace of Good'
  }
  
}

console.log(goodVsEvil("0 5 0 6 1 1", "5 5 5 5 5 8 9"));