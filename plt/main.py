from flask import Flask, Response, send_file, request
import io
import random
import operator
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.figure import Figure
from matplotlib.backends.backend_agg import FigureCanvasAgg as FigureCanvas
from flask_cors import CORS, cross_origin

app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'
plt.style.use('bmh')


@app.route('/plot/<index>', methods=['GET', 'POST'])
@cross_origin()
def plot(index):
    index = int(index)
    config = request.json
    fig = Figure()
    if config:
        fig = create_figure(config, index)
    output = io.BytesIO()
    FigureCanvas(fig).print_png(output)
    return Response(output.getvalue(), mimetype='image/png')


def create_figure(config, index):
    state = State(config)
    state.plot()

    fig = Figure(figsize=(6, 6), dpi=240)
    axis = fig.add_subplot(1, 1, 1)
    axis.set_xlim([state.ranges['emin'] *
                   state.ranges['chans'] /
                   state.ranges['emax'], state.ranges['chans']])
    if index == 0:
        axis.plot(state.plot1[0], state.plot1[1], ',-')
    if index == 1:
        axis.plot(state.plot2[0], state.plot2[1], ',-')
    if index == 2:
        axis.scatter(state.plot3[0], state.plot3[1], s=2)
    if index == 3:
        axis.scatter(state.plot4[0], state.plot4[1], s=2)
    # fig.tight_layout()
    axis.set(xlabel='Channels', ylabel='Intensity', title='')
    return fig


class State():
    passed = True

    def __init__(self, config):
        self.lines = []
        self.background = {}
        self.ranges = {}
        self.config = {'background': True, 'expand': 'FWHM', 'flag': 'first'}

        for line in config['lines']:
            self.lines.append({
                'I': float(line['intensity']),
                'E': float(line['energy']),  # * 1e3,
                'FWHM': float(line['fwhm'])  # * 1e3
            })

        self.lines.sort(key=operator.itemgetter('E'))

        self.background['a'] = float(config['background']['a'])
        self.background['b'] = float(config['background']['b'])
        self.background['e1'] = float(config['background']['e1'])
        self.background['e2'] = float(config['background']['e2'])

        self.ranges['emax'] = float(config['range']['emax'])  # * 1e3
        self.ranges['emin'] = float(config['range']['emin'])  # * 1e3
        self.ranges['chans'] = float(config['range']['chan_number'])

        self.xs = np.linspace(1,
                              int(self.ranges['chans']),
                              int(self.ranges['chans']))
        self.ys = np.zeros(int(self.ranges['chans']))
        self.plot1 = np.vstack((self.xs, self.ys))

    def search(self, my_array, target):
        diff = my_array - target
        mask = np.ma.less_equal(diff, 0)
        if np.all(mask):
            return None
        mask_diff = np.ma.masked_array(diff, mask)
        return mask_diff.argmin()

    def plot(self):
        for line in self.lines:
            it = self.search(
                self.plot1[0],
                (line['E'] * (self.ranges['chans']) / (self.ranges['emax']))
            )
            self.plot1[1][it] += line['I']
        self.plot2 = np.array(self.plot1)
        if(self.config['background']):
            self.plot2[1] += (self.background['e1'] *
                              np.exp(-1 * self.background['e2'] * self.plot2[0]))
            self.plot2[1] += (self.background['a'] *
                              self.plot2[0] + self.background['b'])
        elif(self.config['background'] == False):
            None
        else:
            passed = False
            print("Error: background value in config file is undefined")

        self.plot3 = np.array(self.plot2)

        if(self.config['expand'] == 'FWHM'):
            for line in self.lines:
                sig = line['FWHM'] / 2.355 * \
                    self.ranges['chans'] / self.ranges['emax']
                gaus = (np.exp(-1 * np.power(self.plot3[0] - (
                    line['E'] * self.ranges['chans'] / self.ranges['emax']), 2) / (2 * np.power(sig, 2))))
                self.plot3[1] += line['I'] * gaus

        elif(self.config['expand'] == 'None'):
            for line in self.lines:
                it = self.search(
                    self.plot3[0],
                    (line['E'] *
                     self.ranges['chans'] /
                        self.ranges['emax']))
                self.plot3[1][it] += line['I']
        else:
            passed = False
            print("Error: expand value in config file is undefined")

        self.plot4 = np.array(self.plot3)
        passed_inner = True
        if(self.config['flag'] == 'second'):
            A, B = np.shape(plot4)
            i = 0
            for i in range(B):
                lam = self.plot4[1][i]
                if(lam >= 0):
                    self.plot4[1][i] = np.random.poisson(lam=lam, size=None)
                else:
                    passed_inner = False
        elif(self.config['flag'] == 'first'):
            A, B = np.shape(self.plot4)
            i = 0
            for i in range(B):
                if(self.plot4[1][i] > 10):
                    mu = self.plot4[1][i]
                    sigma = np.sqrt(self.plot4[1][i])
                    self.plot4[1][i] = random.gauss(mu, sigma)
                else:
                    lam = self.plot4[1][i]
                    if(lam >= 0):
                        self.plot4[1][i] = np.random.poisson(lam=lam,
                                                             size=None)
                    else:
                        passed_inner = False
        elif(self.config['flag'] == 'None'):
            None
        else:
            passed = False
            print("Error: flag value in config file is undefined")
