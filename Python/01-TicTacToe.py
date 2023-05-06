global NombreJugador1
NombreJugador1 = 'Jugador 1'
global NombreJugador2
NombreJugador2 = 'Jugador 2'

global Simbolo1
Simbolo1 = 'X'
global Simbolo2
Simbolo2 = 'O'

def menu():
    print("\n\tTres en Raya")
    print('1. Nombres de jugadores\n' +
          '2. Cambiar fichas\n' +
          '3. Reglas\n' +
          '4. Jugar\n' +
          '5. Salir')
    return input('Ingrese su opción: ')

def Reglas():
    print("Las reglas del juego son las siguientes: \n" +
        "💠 Cada jugador solo debe colocar su símbolo una vez por turno y no debe ser sobre una casilla ya jugada\n" +
        "💠 El primer jugador que consiga marcar una línea de tres símbolos será el ganador\n" +
        "💠 Se debe conseguir realizar una línea recta o diagonal por símbolo\n" +
        "💠 Cuando todas las casillas estén llenas y nadie marcó 3 en raya será un empate y se iniciará de nuevo el juego")

def NombresJugadores():
    global NombreJugador1
    global NombreJugador2
    print(f'\n\tNombres de jugadores:\n1.{NombreJugador1}\n2.{NombreJugador2}')
    option = input('Cuál quieres cambiar? ')
    if option == '1':
        NombreJugador1 = input('Ingresa el nuevo nombre del jugador 1: ')
    elif option == '2':
        NombreJugador2 = input('Ingresa el nuevo nombre del jugador 2: ')
    else:
        print('Opción inválida. Intente de nuevo')

def SimbolosJugadores():
    global NombreJugador1
    global NombreJugador2
    global Simbolo1
    global Simbolo2
    print(f'\n\tLas fichas para los jugadores son:\n{NombreJugador1}: {Simbolo1}\n{NombreJugador2}: {Simbolo2}')
    option = input('¿Quieres intercambiar los simbolos? [S/N]')
    if option == 'S' or option == 's':
        Simbolo1, Simbolo2 = Simbolo2, Simbolo1
    else:
        print('Las fichas permaneceran iguales')


def crearTablero():
    matriz = []
    filas = 3
    columnas = 3
    for i in range(filas):
        matriz.append(['-'] * columnas)
    dim = (filas,columnas)
    return matriz, dim

def mostrarTablero(matriz, dim):
    filas, columnas = dim
    for i in range(filas):
        for j in range(columnas):
            print(matriz[i][j], end='\t')
        print('')

def llenarTablero(matriz, simbolo):
    filas = int(input('Ingrese fila: '))
    columnas = int(input('Ingrese columna: '))
    if matriz[filas-1][columnas-1] == '-':
        matriz[filas-1][columnas-1] = simbolo
    else:
        print('Casilla ocupada. Intente de nuevo')
        llenarTablero(matriz, simbolo)

def Jugar(Tablero, dim):
    end = False
    count=0
    mostrarTablero(Tablero, dim)
    turno = 1
    while end == False:
        simboloActual = ''
        jugadorActual = ''
        if turno == 1:
            print('\n\tEs turno de ', NombreJugador1)
            simboloActual = Simbolo1
            jugadorActual = NombreJugador1
            llenarTablero(Tablero,Simbolo1)
            turno = 2
            count+=1
        else:
            print('\n\tEs turno de ', NombreJugador2)
            simboloActual = Simbolo2
            jugadorActual = NombreJugador2
            llenarTablero(Tablero, Simbolo2)
            turno = 1
            count+=1
        mostrarTablero(Tablero,dim)

        if Tablero[0][0] == simboloActual and Tablero[0][1] == simboloActual and Tablero[0][2] == simboloActual:
            end = True            
        if Tablero[1][0] == simboloActual and Tablero[1][1] == simboloActual and Tablero[1][2] == simboloActual:           
            end = True        
        if Tablero[2][0] == simboloActual and Tablero[2][1] == simboloActual and Tablero[2][2] == simboloActual:           
            end = True
        if Tablero[0][0] == simboloActual and Tablero[1][0] == simboloActual and Tablero[2][0] == simboloActual:
            end = True            
        if Tablero[0][1] == simboloActual and Tablero[1][1] == simboloActual and Tablero[2][1] == simboloActual:
            end = True     
        if Tablero[0][2] == simboloActual and Tablero[1][2] == simboloActual and Tablero[2][2] == simboloActual:
            end = True
        if Tablero[0][0] == simboloActual and Tablero[1][1] == simboloActual and Tablero[2][2] == simboloActual:
            end = True            
        if Tablero[2][0] == simboloActual and Tablero[1][1] == simboloActual and Tablero[0][2] == simboloActual:
            end = True 
        if count == 9:
            end = True
    if end == True and count == 9:
        print('\n\tJuego Terminado EMPATE')
    else:
        print('\n\tJuego Terminado!\n\t' + jugadorActual + ' ganó!')

def main():
    gameOver = False
    while gameOver == False:
        option = menu()
        if option == '1':
            NombresJugadores()
        elif option == '2':
            SimbolosJugadores()
        elif option == '3':
            Reglas()
        elif option == '4':
            Tablero, dim = crearTablero()
            Jugar(Tablero,dim)
        elif option == '5':
            print('Gracias por jugar. Nos vemos pronto!')
            gameOver = True
        else:
            print('Opción inválida. Ingrese una opción válida')

if __name__ == '__main__':
    main()