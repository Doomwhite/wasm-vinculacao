extern "C" {
    #[link_name = "console_log"]
    fn log(x: u32) -> u32;

    #[link_name = "alert"]
    fn alert(x: u8) -> u8;
}

// Define o nome da biblioteca como "numeros"
// Também configura o tipo da biblioteca como estática
#[link(name = "numeros", kind = "static")]
extern "C" {
    fn retorna_dez() -> i32;

    fn quadrado(x: i32) -> i32;
}

#[no_mangle]
pub fn executar() {
    unsafe {
        let dez = retorna_dez();

        log(20); // 20
        log(87654321); // 87654321

        log(dez); // 10
        log(quadrado(dez)); // 100

        alert(85); // 85 com window.alert
    }
}
