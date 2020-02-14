<template>
    <div class="wines">
        <ul class="nav nav-tabs">
            <li class="nav-item">
                <a
                    href="#red-wines"
                    v-on:click="wine_color = 'red'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'red' }"
                    data-toggle="tab"
                    >Red</a
                >
            </li>
            <li class="nav-item">
                <a
                    href="#white-wines"
                    v-on:click="wine_color = 'white'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'white' }"
                    data-toggle="tab"
                    >White</a
                >
            </li>
            <li class="nav-item">
                <a
                    href="#pink-wines"
                    v-on:click="wine_color = 'pink'"
                    class="nav-link"
                    v-bind:class="{ active: wine_color === 'pink' }"
                    data-toggle="tab"
                    >Pink</a
                >
            </li>
        </ul>
        <div class="tab-content">
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'red' }"
                id="red-wines"
            >
                <v-card>
                    <v-card-title>
                        Red Wines
                        <v-spacer></v-spacer>
                        <v-text-field
                            v-model="search"
                            append-icon="search"
                            label="Search"
                            single-line
                            hide-details
                        ></v-text-field>
                    </v-card-title>
                    <v-data-table
                        :headers="headers"
                        :items="redWines"
                        :items-per-page="15"
                        :search="search"
                        :custom-filter="filterAccentsAndCase"
                        @click:row="openSaqTab"
                        multi-sort
                        class="elevation-1"
                    ></v-data-table>
                </v-card>
            </div>
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'white' }"
                id="white-wines"
            >
                <v-card>
                    <v-card-title>
                        White Wines
                        <v-spacer></v-spacer>
                        <v-text-field
                            v-model="search"
                            append-icon="search"
                            label="Search"
                            single-line
                            hide-details
                        ></v-text-field>
                    </v-card-title>
                    <v-data-table
                        :headers="headers"
                        :items="whiteWines"
                        :items-per-page="15"
                        :search="search"
                        :custom-filter="filterAccentsAndCase"
                        @click:row="openSaqTab"
                        multi-sort
                        class="elevation-1"
                    ></v-data-table>
                </v-card>
            </div>
            <div
                class="tab-pane fade show"
                v-bind:class="{ active: wine_color === 'pink' }"
                id="pink-wines"
            >
                <v-card>
                    <v-card-title>
                        Pink Wines
                        <v-spacer></v-spacer>
                        <v-text-field
                            v-model="search"
                            append-icon="search"
                            label="Search"
                            single-line
                            hide-details
                        ></v-text-field>
                    </v-card-title>
                    <v-data-table
                        :headers="headers"
                        :items="pinkWines"
                        :items-per-page="15"
                        :search="search"
                        :custom-filter="filterAccentsAndCase"
                        @click:row="openSaqTab"
                        multi-sort
                        class="elevation-1"
                    ></v-data-table>
                </v-card>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    props: ['redWines', 'whiteWines', 'pinkWines'],
    data() {
        return {
            openSaqTab: row => {
                window.open(
                    `https://www.saq.com/en/catalogsearch/result/?q=${encodeURIComponent(
                        row.name
                    )}`,
                    '_blank'
                );
            },
            wine_color: 'red',
            search: '',
            filterAccentsAndCase: (value, search, item) => {
                return (
                    value != null &&
                    search != null &&
                    typeof value === 'string' &&
                    value
                        .toString()
                        .toLocaleLowerCase()
                        .normalize('NFD')
                        .replace(/[\u0300-\u036f]/g, '')
                        .indexOf(search) !== -1
                );
            },
            headers: [
                {
                    text: 'Name',
                    value: 'name'
                },
                {
                    text: 'Available Online',
                    value: 'availableOnline'
                },
                {
                    text: 'Country',
                    value: 'country'
                },
                {
                    text: 'Region',
                    value: 'region'
                },
                {
                    text: 'Designation of Origin',
                    value: 'designationOfOrigin'
                },
                {
                    text: 'Producer',
                    value: 'producer'
                },
                {
                    text: 'Volume',
                    value: 'volume',
                    sort: (a, b) => {
                        a = parseInt(a);
                        b = parseInt(b);
                        return a - b;
                    }
                },
                {
                    text: 'Price',
                    value: 'price',
                    sort: (a, b) => {
                        const aIndex = a.indexOf('(');
                        if (aIndex !== -1) {
                            a = parseFloat(a.substring(1, aIndex));
                        } else {
                            a = parseFloat(a.substring(1));
                        }
                        const bIndex = b.indexOf('(');
                        if (bIndex !== -1) {
                            b = parseFloat(b.substring(1, bIndex));
                        } else {
                            b = parseFloat(b.substring(1));
                        }

                        return a - b;
                    }
                },
                {
                    text: 'Rating',
                    value: 'rating'
                }
            ]
        };
    }
};
</script>
