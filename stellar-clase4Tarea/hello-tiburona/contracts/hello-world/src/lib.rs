#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, String, Symbol,
};
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    //¿Por qué retorna Result<(), Error> y no solo ()? R.- Por que esta función puede devolver
    //un valor correcto (Ok(T)) o un error (Err(E)).
    //¿Qué podría salir mal en una inicialización? R.- Ejecutar la funcion initialize mas de una vez
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }
        // ¿Por qué instance storage para Admin?
        // R.- Porque el admin pertenece al estado interno y persistente del contrato
        // no al entorno global ni temporal
        env.storage().instance().set(&DataKey::Admin, &admin);

        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        // ¿Qué significan los dos 100?
        // R.- El tiempo de vida (TTL)en unidades de ledgers y controlan cuándo
        // y cuánto se extiende la vida del estado del contrato
        env.storage().instance().extend_ttl(100, 100);

        Ok(())
    }
    // ¿Por qué retorna Result<Symbol, Error> en lugar de solo Symbol?
    // R.- Por que no podria comunicarnos el error en caso de que ocurriese uno
    // ya que no es solamente una funcion de lectura
    pub fn hello(env: Env, usuario: Address, nombre: String) -> Result<Symbol, Error> {
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        //¿Por qué validar la longitud antes de tocar storage?
        //R.- Por que las operaciones de escritura son muy costosas
        //antes de modificar el storage y gastar fee, es bueno realizar
        //estas valdiciones que cuaestan 0 fee

        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }

        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage().instance().get(&key_contador).unwrap_or(0);

        env.storage().instance().set(&key_contador, &(contador + 1));

        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);

        env.storage().instance().extend_ttl(100, 100);

        Ok(Symbol::new(&env, "Hola"))
    }

    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }
    // Funcion para resetar el contador variable de entrad importante address de admin
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    // NOTA: El test debe hacer panic, cambie expected, por que el sdk
    // lo convierte a "Error(Contract, #4)" su valor en su tipo enum
    #[test]
    #[should_panic(expected = "4")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client: HelloContractClient<'_> = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);
        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    // NOTA: El test debe hacer panic, cambie expected, por que el sdk
    // lo convierte a "Error(Contract, #1)" su valor en su tipo enum
    #[test]
    #[should_panic(expected = "1")]
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);
    }

    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        // Comentamos otro por que no se esta usando, y ya existe otro test que
        // intenta resetar con user no admin, aunque seria mejor quitarlo jeje
        //let otro = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    // NOTA: El test debe hacer panic, cambie expected, por que el sdk
    // lo convierte a "Error(Contract, #3)" su valor en su tipo enum
    #[test]
    #[should_panic(expected = "3")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        client.initialize(&admin);
        client.reset_contador(&otro);
    }
}
