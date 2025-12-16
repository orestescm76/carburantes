# Carburantes
Aplicación para determinar los precios de los carburantes, en Rust
Application to see the petrol and diesel prices in Spain, made with Rust.

Carburantes is fuels in Spanish

## Usage
- -c, --city, --ciudad [city] Checks the fuel prices in the specified city.
- --fuel-type, --combustible [diesel, dieselp, gas, gas98] Selects a specific fuel, Diesel, Diesel Premium, Gasoline (95 octane), 98 octane gasoline
- -m, --money, --dinero [money] Outputs the amount of liters you fuel with the money specified. This has priority.
- -l, --liters, --litros [liters]. Outputs the money you need to refuel that amount of liters. If money is specified this is ignored. <br>
Example output:
```Precios de carburantes en linares
ALLENOIL, 1.259 €/l
GASOLINERAS HF, 1.259 €/l
ALCAMPO, 1.275 €/l
S.C.A. SANAGUSTIN, 1.339 €/l
GASOLINERA SANTANA, 1.339 €/l
MLC, 1.479 €/l
BP LINARES, 1.489 €/l
BP OIL ESPAÑA E.S.SAN GONZALO, 1.489 €/l
MOEVE, 1.494 €/l
REPSOL, 1.499 €/l
CEPSA, 1.509 €/l
REPSOL, 1.509 €/l
```
