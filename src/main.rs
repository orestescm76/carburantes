use reqwest;
use json::{self, JsonValue};
use std::cmp::Ordering;
use std::option::Option;
use std::collections::HashMap;
use std::fs::{self, exists};
use clap::{Parser,ValueEnum};

#[derive(Parser)]
struct Cli{
	#[arg(short, long)]
	city: String,
	#[arg(value_enum, short, long)]
	fuel_type: FuelType,
	#[arg(short, long)]
	money: Option<f32>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FuelType {
	Gas,
	Diesel
}

fn save_initial_file(url: &str) {
	let req = reqwest::blocking::get(url.to_string() + "/Listados/Municipios/");
	let out = req.unwrap().text().unwrap();
	let _ = fs::write("cities.json", out);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
	//sacamos los json
	let url_base = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes";
	match exists("cities.json") {
		Ok(false) => save_initial_file(url_base),
		Ok(true) => (),
		Err(_) => (),
	}
	//municipios
	let file = std::fs::read_to_string("cities.json")?;
	let json_municipios = json::parse(&file).unwrap();
	let mut municipios = HashMap::new();
	for mun in json_municipios.members() {
		let muni = get_municipio(&mun);
	 	municipios.insert(muni.municipio.clone(), muni);
	}
	let args = Cli::parse();
	// //json
	let mut url_precios = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/EstacionesTerrestres/FiltroMunicipio/".to_string();
	let query = args.city.to_lowercase().trim().to_string();
	let mun = municipios.get(&query).expect("city not found, exiting");
	url_precios.push_str(&mun.id.to_string().as_str());
	//precios
	let mut result = reqwest::blocking::get(url_precios)?.text()?;
	//sacamos el objeto json
	result = result.replace("Precio Gasolina 95 E5", "Precio95");
	result = result.replace("Precio Gasoleo A", "PrecioGasoil");
	let json_out = json::parse(&result).unwrap();
	//precios
	let gasolineras = get_precios(&json_out).unwrap();
	let mut resultados : Vec<Gasolinera> = Vec::new();
	for gasolinera in gasolineras.members() {
		resultados.push(get_gasolinera(&gasolinera));
	}
	sort_gas(&mut resultados, args.fuel_type);
	print_prices(mun, &resultados,args.fuel_type ,args.money );
	Ok(())
}
fn print_prices(mun: &Municipio, gas_stations :&Vec<Gasolinera>, f_type: FuelType, money: Option<f32>) {
	//print
	println!("Precios de carburantes en {}",mun.municipio);
	for gas in gas_stations {
		let price: f32 = match f_type {
			FuelType::Gas => gas.precio_gasolina,
			FuelType::Diesel => gas.precio_gasoil,
		};
		match money {
			Some(m) => println!("{}, {} €/l; {} l", gas.nombre, price, m/price),
			None =>  println!("{}, {} €/l", gas.nombre, price),
		}
	}
}
fn sort_gas(gas_stations :&mut Vec<Gasolinera>, f_type: FuelType) {
	match f_type {
		FuelType::Gas => 	gas_stations.sort_by(|a,b| {
			a.precio_gasolina
			.partial_cmp(&b.precio_gasolina)
			.unwrap_or(Ordering::Equal)
		}),
		FuelType::Diesel => gas_stations.sort_by(|a,b| {
			a.precio_gasoil
			.partial_cmp(&b.precio_gasoil)
			.unwrap_or(Ordering::Equal)
		}),
	}
}
// resultados.push(get_gasolinera(&resultados));
// println!("{:?}", get_gasolinera(&gasolinera));
fn get_precios(raiz : &JsonValue) -> Option<JsonValue> {
	let mut lista_precios=None;
	for val in raiz.entries() {
		if val.0 == "ListaEESSPrecio" {
			lista_precios = Some(val.1.clone());
		}
	}

	lista_precios
}

fn get_gasolinera(json_val : &JsonValue) -> Gasolinera {
	let mut gas = Gasolinera {

		direccion : "".to_string(),
		horario : "".to_string(),
		nombre : "".to_string(),
		precio_gasoil : 0.0,
		precio_gasolina : 0.0,
	};

	for entry in json_val.entries() {
		match entry.0 {
			"Dirección" => gas.direccion = entry.1.as_str().unwrap().to_string(),
			"Horario" => gas.horario = entry.1.as_str().unwrap().to_string(),
			"PrecioGasoil" => gas.precio_gasoil = entry.1.as_str().unwrap().replace(",",".").parse::<f32>().unwrap_or(0.0),
			"Precio95" => gas.precio_gasolina = entry.1.as_str().unwrap().replace(",",".").parse::<f32>().unwrap_or(0.0),
			"Rótulo" => gas.nombre = entry.1.as_str().unwrap().to_string(),
			_ => ()
		}
	}
	gas
}
fn get_municipio(json_val:&JsonValue) -> Municipio {
	let mut mun = Municipio {
		ccaa : "".to_string(),
		municipio : "".to_string(),
		provincia : "".to_string(),
		id:0
	};
	for entry in json_val.entries() {
		match entry.0 {
			"CCAA" => mun.ccaa = entry.1.as_str().unwrap().to_string(),
			"Municipio" => mun.municipio = entry.1.as_str().unwrap().to_string().to_lowercase(),
			"Provincia" => mun.provincia = entry.1.as_str().unwrap().to_string(),
			"IDMunicipio" => mun.id = entry.1.as_str().unwrap().parse::<i32>().unwrap_or(0),
			_ => ()
		}
	}
	mun
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Gasolinera {
	nombre : String,
	precio_gasolina : f32,
	precio_gasoil: f32, 
	direccion: String,
	horario: String,
}

#[derive(Debug)]
struct Municipio {
	ccaa:String,
	municipio:String,
	provincia:String,
	id:i32,
}
