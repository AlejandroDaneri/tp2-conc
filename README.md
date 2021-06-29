# TP2 - Buscador de Sinónimos Rústico

## Integrantes

- Daneri, Alejandro
- Lafroce, Matias

## Ejecución

El programa recibe 3 parámetros de entrada:

- **file**: nombre del archivo que contiene las palabras a buscar
- **max_conc_reqs**: cantidad máxima de requests HTTP a procesar en forma concurrente para todos los sitios
- **page_cooldown**: tiempo mínimo de espera entre dos requests HTTP sucesivos para un mismo sitio

A su vez se debe especificar que binario se quiere generar, teniendo dos opciones: actores y sinonimos. Por ejemplo:

> cargo run --bin actores words.txt 5 5
