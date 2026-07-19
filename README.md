# Lab 1 - Filling any polygon

Este repositorio contiene la implementación del Laboratorio 1. El objetivo principal es rellenar múltiples polígonos utilizando algoritmos de rasterización construidos desde cero en Rust.

### Autor: Pedro Caso - 241286

## Cómo ejecutar el proyecto

1. Instalar Rust en caso de no tenerlo en la computadora.
2. Clona este repositorio y abre una terminal en la carpeta raíz 
3. Ejecuta el siguiente comando para compilar y correr el programa:
   ```bash
   cargo run
   ```
4. Al finalizar, el programa generará un archivo llamado `out.bmp` en la misma carpeta.
5. Abre `out.bmp` con cualquier visor de imágenes para ver el resultado final.

## Algoritmo Utilizado

El proyecto utiliza el algoritmo de **Scanline Polygon Fill** combinado con la regla de paridad **Even-Odd**. 
Este enfoque permite:
- Rellenar polígonos complejos de más de 4 vértices.
- Manejar polígonos con agujeros internos, procesando simultáneamente las aristas exteriores e interiores durante el barrido horizontal.
- Dibujar los contornos utilizando el algoritmo de Bresenham para el trazado de líneas.

## Estructura de Ramas (Branches)

Este repositorio está estructurado siguiendo la rúbrica del laboratorio:
- `Poligon-1`: Dibuja únicamente el Polígono 1 (Amarillo con borde blanco).
- `Poligon-2`: Dibuja únicamente el Polígono 2 (Azul con borde blanco).
- `Poligon-3`: Dibuja únicamente el Polígono 3 (Rojo con borde blanco).
- `Poligon-4`: Dibuja el Polígono 4 y utiliza el Polígono 5 como un agujero central (Verde con bordes blancos).
- `main`: Es la integración final. Contiene los commits de merge de todas las ramas anteriores y genera una imagen con todos los polígonos dibujados simultáneamente.

## Notas adicionales
- El archivo de salida nativo es `.bmp`, el cual almacena los pixeles en formato BGR.
- El proyecto no incluye carpetas de compilación, ya que están ignoradas por `.gitignore` y se generaran automaticamente al ejecutar el proyecto de forma local.
